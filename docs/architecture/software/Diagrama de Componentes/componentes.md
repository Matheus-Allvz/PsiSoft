# 3. Arquitetura de Componentes (Nível C3)

## 3.1 Diagrama de Componentes

A figura abaixo apresenta o Diagrama de Componentes do sistema PsiSoft, evidenciando a decomposição lógica da aplicação em módulos de software, suas interfaces fornecidas e requeridas, bem como as integrações com serviços externos necessários ao funcionamento da plataforma.

### Justificativa Arquitetural

A arquitetura foi estruturada seguindo os princípios de baixo acoplamento e alta coesão descritos por Craig Larman em *Applying UML and Patterns*. Cada componente encapsula responsabilidades específicas do domínio de negócio, comunicando-se por meio de interfaces bem definidas.

O componente **Frontend UI** atua como ponto de entrada para os usuários da plataforma, consumindo os serviços de Agendamento e Prontuário. O componente **Agendamento** é responsável pelo gerenciamento de reservas e validações profissionais junto ao Cadastro e-Psi. O componente **Consulta** centraliza o fluxo clínico, integrando-se aos módulos de Gestão Financeira e Notificação para execução dos contratos de operação definidos na análise de requisitos.

A persistência dos dados é realizada por meio do componente **Banco de Dados**, acessado exclusivamente pelos componentes autorizados, garantindo isolamento e rastreabilidade das informações clínicas. O componente **Prontuário** integra-se ao Motor de IA para automatizar a transcrição de atendimentos, enquanto a Gestão Financeira comunica-se com o Gateway de Pagamento para processamento e confirmação de cobranças.

A arquitetura também incorpora requisitos não funcionais críticos, incluindo criptografia AES-256 para proteção de dados sensíveis, comunicação segura via HTTPS/TLS 1.3 e armazenamento de prontuários em conformidade com as resoluções do Conselho Federal de Psicologia.

---

## Apêndice: Código-Fonte do Diagrama (PlantUML)

**Nota técnica:** Este bloco de código serve para manutenção futura da arquitetura pela equipe e pode ser omitido na compilação final do LaTeX.

```plantuml
@startuml diagrama-de-componentes
' ============================================================
' Diagrama de Componentes – PsiSoft (Nível C3)
' Referência: Craig Larman, Applying UML and Patterns, Cap. 33
' Disciplina: Arquitetura e Desenho de Software (CIC1060) – PUC-GO
' ============================================================

skinparam defaultTextAlignment center
skinparam component {
    BackgroundColor #EFF4FB
    BorderColor #2C5F9E
    FontName Arial
}
skinparam interface {
    BackgroundColor #FFFFFF
    BorderColor #2C5F9E
}
skinparam rectangle {
    BackgroundColor #F0F8FF
    BorderColor #888888
    RoundCorner 12
}
skinparam note {
    BackgroundColor #FFFDE7
    BorderColor #F9A825
}
skinparam ArrowColor #444444
skinparam linetype ortho

rectangle "Plataforma PsiSoft" as PsiSoft {

    component [Frontend UI] as UI
    component [Agendamento] as Agendamento
    component [Consulta] as Consulta
    component [Gestão Financeira] as Financeiro
    component [Notificação] as Notificacao
    component [Prontuário] as Prontuario
    component [Banco de Dados] as BD

    interface "IAgendamento" as IAgendamento
    interface "IConsulta" as IConsulta
    interface "IFinanceiro" as IFinanceiro
    interface "INotificacao" as INotificacao
    interface "IProntuario" as IProntuario
    interface "IBancoDeDados" as IBD

    Agendamento - IAgendamento
    Consulta - IConsulta
    Financeiro - IFinanceiro
    Notificacao - INotificacao
    Prontuario - IProntuario
    BD - IBD

    UI ..> IAgendamento : <<use>>
    UI ..> IProntuario : <<use>>

    Agendamento ..> IConsulta : <<use>>
    Consulta ..> IFinanceiro : <<use>>
    Consulta ..> INotificacao : <<use>>

    Consulta ..> IBD : <<use>>
    Prontuario ..> IBD : <<use>>
}

rectangle "Serviços Externos" as Externos {

    component [Gateway de Pagamento] as Gateway
    component [API WhatsApp\n(Meta)] as WhatsApp
    component [API e-Psi\n(CFP)] as ePsi
    component [Motor de IA\n(Transcrição)] as MotorIA

    interface "IGatewayPagamento" as IGateway
    interface "IWhatsApp" as IWhatsApp
    interface "IePsi" as IePsi
    interface "IMotorIA" as IMotorIA

    Gateway - IGateway
    WhatsApp - IWhatsApp
    ePsi - IePsi
    MotorIA - IMotorIA
}

Financeiro ..> IGateway : <<use>>
Notificacao ..> IWhatsApp : <<use>>
Agendamento ..> IePsi : <<use>>
Prontuario ..> IMotorIA : <<use>>

note right of Agendamento
  processarReserva()
  registrarConsultaPendente()
  Contrato 01 – Larman
  RF-04 | RF-05
end note

note right of Financeiro
  gerarOrdemCobranca()
  efetivarConsulta() via Webhook
  Contratos 02 e 03 – Larman
  RNF-01 (HTTPS/TLS 1.3)
end note

note right of Prontuario
  Guarda obrigatória: 5 anos (RN-01)
  Conforme Res. CFP 001/2009 (RNF-07)
  Criptografia AES-256 (RNF-01)
end note

note right of BD
  Rede privada (VPC) – isolado
  Criptografia AES-256 em repouso
  Backup automático (RNF-03)
end note

@enduml
```
