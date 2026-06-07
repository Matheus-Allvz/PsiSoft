## 2. Modelos de Componentes e Comportamentos (Nível C3)

### 2.1 Diagrama de Sequência de Componentes: Fluxo de Agendamento e Pagamento

A figura abaixo ilustra as interações e trocas de mensagens entre os componentes arquiteturais do sistema durante o processo central de reserva de horário e processamento assíncrono de pagamento.

**Justificativa Arquitetural:**
O diagrama demonstra o comportamento dinâmico dos pacotes definidos no Nível C2 (`Agendamento`, `Consulta`, `Gestão Financeira` e `Notificação`). Optou-se por um modelo de comunicação assíncrona (Webhook) na etapa de confirmação de pagamento para evitar bloqueios de processamento (*polling*), otimizando o consumo de recursos na nuvem. Toda a comunicação externa com o Gateway e WhatsApp ocorre apenas após as devidas transições de estado nas entidades de domínio locais.

### Apêndice: Código-Fonte do Diagrama (PlantUML)

*Nota técnica: Este bloco de código serve para manutenção futura da arquitetura pela equipe e pode ser omitido na compilação final do LaTeX.*

```plantuml
@startuml
' Configurações visuais padronizadas
skinparam defaultTextAlignment center
skinparam sequence {
    ParticipantBackgroundColor LightBlue
    ParticipantBorderColor DarkBlue
    LifeLineBorderColor DarkBlue
    ArrowColor DarkBlue
}

actor "Paciente" as Paciente

box "Plataforma PsiSoft (Módulos Internos)" #F0F8FF
    participant "Interface\n(Frontend UI)" as UI
    participant "Componente\nAgendamento" as C_Agendamento
    participant "Componente\nConsulta" as C_Consulta
    participant "Componente\nGestão Financeira" as C_Financeiro
    participant "Componente\nNotificação" as C_Notificacao
end box

box "Serviços Externos" #F9F9F9
    participant "Gateway de\nPagamento" as Ext_Gateway
    participant "API do\nWhatsApp" as Ext_Wpp
end box

' --- Início do Fluxo ---
autonumber

Paciente -> UI : Solicita reserva de horário (idPsicologo, dataHora)
UI -> C_Agendamento : processarReserva(idPsicologo, dataHora)

activate C_Agendamento
    C_Agendamento -> C_Consulta : registrarConsultaPendente(dadosConsulta)
    C_Consulta --> C_Agendamento : Retorna idConsulta
    
    C_Agendamento -> C_Financeiro : gerarOrdemCobranca(idConsulta, valor)
    
    activate C_Financeiro
        C_Financeiro -> Ext_Gateway : solicitarLinkPagamento(Pix/Cartao)
        Ext_Gateway --> C_Financeiro : Retorna Link/QR Code
        C_Financeiro --> C_Agendamento : Retorna dados de cobrança
    deactivate C_Financeiro
    
    C_Agendamento --> UI : Exibe tela de pagamento
deactivate C_Agendamento

' --- Fluxo de Confirmação (Assíncrono via Webhook) ---
note over Ext_Gateway, C_Financeiro : O paciente realiza o pagamento no aplicativo do seu banco

Ext_Gateway -> C_Financeiro : Webhook: notificacaoPagamentoAprovado(idPagamento)
activate C_Financeiro
    
    C_Financeiro -> C_Financeiro : atualizarStatus(quitado = True)
    C_Financeiro -> C_Consulta : efetivarConsulta(idConsulta)
    
    activate C_Consulta
        C_Consulta -> C_Consulta : atualizarStatus(presenca = Agendada)
        C_Consulta -> C_Notificacao : agendarLembreteWpp(idConsulta)
        
        activate C_Notificacao
            C_Notificacao -> Ext_Wpp : dispararConfirmacaoImediata(celular, msg)
        deactivate C_Notificacao
        
        C_Consulta --> C_Financeiro : Consulta Efetivada
    deactivate C_Consulta
    
    C_Financeiro --> Ext_Gateway : HTTP 200 OK (Ciente)   
    C_Financeiro -> UI : Evento WebSocket/SSE (Pagamento Confirmado)
deactivate C_Financeiro

UI -> Paciente : Exibe tela "Consulta Confirmada" e notifica via WhatsApp
@enduml