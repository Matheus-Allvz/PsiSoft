## 4. Diagramas de Sequência (Nível C4)

Este documento descreve os fluxos de implementação detalhados para os principais casos de uso do sistema PsiSoft, mapeando a interação desde a interface do usuário até o banco de dados. A arquitetura de implementação segue o modelo em camadas do Rust: **Axum (Handlers / API)** -> **Service (Lógica de Negócios)** -> **Sqlx (Repositório / Banco de Dados)**.

### 4.1 Cadastrar Paciente

Este diagrama detalha o processo completo de cadastro de um novo paciente, rastreado a partir de `cadastro-paciente.html` invocando a API Rust Axum.

```plantuml
@startuml
skinparam defaultTextAlignment center
skinparam sequence {
    ParticipantBackgroundColor LightBlue
    ParticipantBorderColor DarkBlue
    LifeLineBorderColor DarkBlue
    ArrowColor DarkBlue
}

actor "Usuário/Psicólogo" as User
participant "cadastro-paciente.html\n(Frontend)" as UI
participant "Axum Router\n(api::paciente)" as Router
participant "API Handler\n(create_paciente_handler)" as Handler
participant "Service\n(services::paciente)" as Service
database "PostgreSQL\n(PgPool)" as DB

User -> UI : Preenche dados e clica em Salvar
UI -> Router : POST http://localhost:3000/pacientes/novo\n(JSON: CadastroPacienteRequest)
Router -> Handler : Delega para create_paciente_handler
activate Handler
    Handler -> Service : cadastrar_paciente(&state.db, payload)
    activate Service
        Service -> DB : pool.begin() (Abre transação)
        Service -> Service : hash_password(senha ou "123456")
        
        Service -> DB : INSERT INTO Usuario (nome, login, senha_hash, perfil...)\nRETURNING id
        DB --> Service : usuario_id
        
        Service -> DB : INSERT INTO Paciente (fk_usuario_id, fk_psicologo_id...)\nRETURNING id
        DB --> Service : paciente_id
        
        opt payload.responsavel_nome não vazio
            Service -> DB : INSERT INTO ResponsavelLegal (...)
        end
        
        Service -> DB : INSERT INTO ConfiguracaoPaciente (...)
        Service -> DB : INSERT INTO ConsentimentoPaciente (...)
        
        Service -> DB : tx.commit()
        
        Service -> Service : tokio::spawn(enviar_email_resend)
        Service --> Handler : Result<Ok(paciente_id)>
    deactivate Service
    
    Handler --> Router : StatusCode::CREATED, JSON { "id": paciente_id }
deactivate Handler
Router --> UI : 201 Created
UI -> User : Exibe sucesso e limpa formulário
@enduml
```

### 4.2 Marcar Agendamento

Este diagrama mapeia rigorosamente a lógica de negócio associada ao agendamento de consultas a partir da interface `agendamentos.html`.

```plantuml
@startuml
skinparam defaultTextAlignment center
skinparam sequence {
    ParticipantBackgroundColor LightBlue
    ParticipantBorderColor DarkBlue
    LifeLineBorderColor DarkBlue
    ArrowColor DarkBlue
}

actor "Paciente / Secretária" as User
participant "agendamentos.html\n(Frontend)" as UI
participant "Axum Router\n(api::agendamento)" as Router
participant "API Handler\n(create_agendamento_handler)" as Handler
participant "Service\n(services::agendamento)" as Service
database "PostgreSQL\n(PgPool)" as DB

User -> UI : Seleciona data, horário, psicólogo e salva
UI -> Router : POST http://localhost:3000/agendamentos/novo\n(JSON: CreateAgendamentoRequest)
Router -> Handler : Delega para create_agendamento_handler
activate Handler
    Handler -> Service : criar_agendamento(&state.db, fk_paciente, fk_psicologo, data_hora, modalidade)
    activate Service
        opt modalidade == ModalidadeConsulta::Online
            Service -> DB : SELECT crp, validador_epsi... FROM Psicologo WHERE fk_usuario_id = $1
            DB --> Service : Option<Psicologo>
            alt Psicologo não encontrado
                Service --> Handler : Err(AgendamentoError::PsicologoNaoEncontrado)
            else validador_epsi is None
                Service --> Handler : Err(AgendamentoError::EpsiNaoValidado)
            end
        end
        
        Service -> DB : SELECT id FROM Consulta WHERE fk_psicologo_id = $1 AND data_hora = $2 AND status != 'Cancelada'
        DB --> Service : Option<i32> (conflito)
        alt conflito.is_some()
            Service --> Handler : Err(AgendamentoError::ConflitoHorario)
        end
        
        Service -> DB : INSERT INTO Consulta (fk_paciente, fk_psicologo, data_hora, modalidade, status='Agendada')\nRETURNING *
        DB --> Service : Consulta criada
        
        Service --> Handler : Result<Ok(Consulta)>
    deactivate Service
    
    Handler --> Router : StatusCode::CREATED, JSON(Consulta)
deactivate Handler
Router --> UI : 201 Created
UI -> User : Atualiza tabela de agendamentos na tela
@enduml
```
