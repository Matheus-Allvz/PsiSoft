## 2. Modelos de Componentes e Comportamentos (Nível C3)

### 2.1 Diagrama de Sequência de Componentes: Fluxo de Agendamento e Pagamento

A figura abaixo ilustra as interações e trocas de mensagens entre as camadas arquiteturais do sistema (Rust) durante o processo central de reserva de horário e processamento assíncrono de pagamento.

**Justificativa Arquitetural:**
O diagrama demonstra o comportamento dinâmico das camadas da aplicação (`api`, `services`, `domain`, `events`). Optou-se por separar a lógica de negócios na camada `services`, enquanto as regras de estado e validações residem em `domain`. A comunicação com serviços externos (Gateway e WhatsApp) e as notificações em tempo real ocorrem de forma assíncrona, orquestradas pela camada `events` (usando Tokio), otimizando a resposta da API em Axum e evitando bloqueios no fluxo principal.

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

box "Cliente" #E6F3FF
    participant "Interface\n(Frontend UI)" as UI
end box

box "Plataforma PsiSoft (Backend Rust)" #F0F8FF
    participant "Camada API\n(Axum Handlers)" as API
    participant "Camada Services\n(Business Logic)" as Services
    participant "Camada Domain\n(Models/Entities)" as Domain
    participant "Camada Events\n(Async/Tokio)" as Events
end box

box "Serviços Externos" #F9F9F9
    participant "Gateway de\nPagamento" as Ext_Gateway
    participant "API do\nWhatsApp" as Ext_Wpp
end box

' --- Início do Fluxo ---
autonumber

Paciente -> UI : Solicita reserva de horário (idPsicologo, dataHora)
UI -> API : POST /agendamentos (idPsicologo, dataHora)

activate API
    API -> Services : processar_reserva(dados)
    
    activate Services
        Services -> Domain : Consulta::nova_pendente(dados)
        Domain --> Services : Retorna Entidade Consulta
        
        Services -> Ext_Gateway : solicitar_link_pagamento(valor)
        Ext_Gateway --> Services : Retorna Link/QR Code
        
        Services -> Domain : atualizar_com_cobranca(link)
        Domain --> Services : Entidade Atualizada
        
        Services --> API : Resposta (Consulta + Link)
    deactivate Services
    
    API --> UI : 200 OK (JSON com Link)
deactivate API

UI --> Paciente : Exibe tela de pagamento

' --- Fluxo de Confirmação (Assíncrono via Webhook) ---
note over Ext_Gateway, API : O paciente realiza o pagamento no aplicativo do seu banco

Ext_Gateway -> API : POST /webhooks/pagamento (idPagamento)
activate API
    
    API -> Services : confirmar_pagamento(idPagamento)
    activate Services
        Services -> Domain : Consulta::efetivar()
        Domain --> Services : Estado Atualizado (Quitado/Agendada)
        
        Services -> Events : emitir_evento(ConsultaEfetivada)
        activate Events
            Events -> Ext_Wpp : disparar_notificacao_whatsapp(msg)
            Events -> UI : Evento WebSocket/SSE (Pagamento Confirmado)
        deactivate Events
        
        Services --> API : Sucesso
    deactivate Services
    
    API --> Ext_Gateway : HTTP 200 OK (Ciente)
deactivate API

UI --> Paciente : Exibe tela "Consulta Confirmada" e notifica via WhatsApp
@enduml
```