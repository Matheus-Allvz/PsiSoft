## 4. Diagramas de Sequência (Nível C4)

Este documento descreve os fluxos de implementação detalhados para os principais casos de uso do sistema PsiSoft, mapeando a interação desde a interface do usuário até o banco de dados. A arquitetura de implementação segue o modelo em camadas do Rust: **Axum (Handlers / API)** -> **Service (Lógica de Negócios)** -> **Sqlx (Repositório / Banco de Dados)**.

### 4.1 Fluxo de Autenticação (Login)

Este diagrama detalha o processo de login de um usuário na plataforma.

```plantuml
@startuml
skinparam defaultTextAlignment center
skinparam sequence {
    ParticipantBackgroundColor LightBlue
    ParticipantBorderColor DarkBlue
    LifeLineBorderColor DarkBlue
    ArrowColor DarkBlue
}

actor "Usuário" as User
participant "Frontend UI\n(HTML/JS)" as UI
participant "Axum Handler\n(api::auth_handler)" as Handler
participant "Service\n(services::auth_service)" as Service
participant "Sqlx Repository\n(domain::user_repo)" as Repo
database "PostgreSQL" as DB

User -> UI : Preenche e-mail e senha
UI -> Handler : POST /api/auth/login (JSON)
activate Handler
    Handler -> Service : authenticate(email, password)
    activate Service
        Service -> Repo : find_by_email(email)
        activate Repo
            Repo -> DB : SELECT * FROM users WHERE email = $1
            DB --> Repo : Retorna registro do usuário
            Repo --> Service : User entity
        deactivate Repo
        
        Service -> Service : verifica_hash(password, hash)
        Service -> Service : gerar_jwt(user.id)
        
        Service --> Handler : Retorna Token JWT
    deactivate Service
    
    Handler --> UI : 200 OK (Token JWT)
deactivate Handler

UI -> User : Redireciona para /agendamentos.html
@enduml
```

### 4.2 Fluxo de Novo Agendamento

Este diagrama detalha o processo de criação de um novo agendamento, acionado através da tela `agendamentos.html`.

```plantuml
@startuml
skinparam defaultTextAlignment center
skinparam sequence {
    ParticipantBackgroundColor LightBlue
    ParticipantBorderColor DarkBlue
    LifeLineBorderColor DarkBlue
    ArrowColor DarkBlue
}

actor "Paciente / Usuário" as User
participant "agendamentos.html\n(Frontend)" as UI
participant "Axum Handler\n(api::agendamento_handler)" as Handler
participant "Service\n(services::agendamento_service)" as Service
participant "Sqlx Repository\n(domain::agendamento_repo)" as Repo
database "PostgreSQL" as DB

User -> UI : Clica em "Novo Agendamento" e preenche form
UI -> Handler : POST /api/agendamentos (JSON + JWT)
activate Handler
    Handler -> Handler : valida_auth_jwt()
    Handler -> Service : criar_agendamento(payload)
    activate Service
        Service -> Service : aplicar_regras_negocio(disponibilidade)
        
        Service -> Repo : insert(agendamento)
        activate Repo
            Repo -> DB : INSERT INTO agendamentos (...) RETURNING id
            DB --> Repo : ID do agendamento criado
            Repo --> Service : Agendamento salvo
        deactivate Repo
        
        Service --> Handler : Result<Agendamento, Error>
    deactivate Service
    
    Handler --> UI : 201 Created (JSON)
deactivate Handler

UI -> User : Atualiza tabela de agendamentos na tela
@enduml
```
