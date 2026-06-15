# 4.3 Diagramas de Classes de Implementação

Estes diagramas refletem as estruturas (`structs`) e enumerações (`enums`) da camada de Domínio e Serviços implementadas em Rust, com tipagem estrita (e.g., `String`, `i32`, `NaiveDate`, `Option<T>`).

## Domínio: Usuário (`domain::user`)

```plantuml
@startuml
skinparam classAttributeIconSize 0

class Usuario <<struct>> {
    + id: i32
    + nome: String
    + login: String
    + senha_hash: String
    + perfil: PerfilUsuario
    + email: Option<String>
    + telefone: Option<String>
    + status: bool
}

enum PerfilUsuario <<enum>> {
    Psicologo
    Secretaria
    Contador
    Paciente
}

class Psicologo <<struct>> {
    + crp: String
    + validador_epsi: Option<String>
    + fk_usuario_id: i32
}

Usuario --> PerfilUsuario
Psicologo --> Usuario : fk_usuario_id
@enduml
```

## Domínio: Paciente (`domain::paciente`)

```plantuml
@startuml
skinparam classAttributeIconSize 0

class Paciente <<struct>> {
    + id: i32
    + fk_usuario_id: i32
    + fk_psicologo_id: i32
    + nome: String
    + cpf: String
    + data_nascimento: NaiveDate
    + genero: Option<String>
    + estado_civil: Option<String>
    + telefone: Option<String>
    + telefone_alt: Option<String>
    + endereco: Option<String>
    + criado_em: Option<DateTime<Utc>>
}

class ResponsavelLegal <<struct>> {
    + id: i32
    + fk_paciente_id: i32
    + nome: String
    + cpf: Option<String>
    + parentesco: Option<String>
    + email: Option<String>
    + telefone: Option<String>
}

enum CanalNotificacao <<enum>> {
    Email
    WhatsApp
    SMS
}

enum ModalidadePref <<enum>> {
    Online
    Presencial
    Hibrido
}

class ConfiguracaoPaciente <<struct>> {
    + id: i32
    + fk_paciente_id: i32
    + canal: Option<CanalNotificacao>
    + modalidade: Option<ModalidadePref>
}

class ConsentimentoPaciente <<struct>> {
    + id: i32
    + fk_paciente_id: i32
    + tcle_aceito: Option<bool>
    + transcricao_aceita: Option<bool>
    + chatbot_aceito: Option<bool>
    + metodo_assinatura: Option<String>
    + data_hora_aceite: Option<DateTime<Utc>>
    + ip_assinante: Option<String>
}

Paciente --> ResponsavelLegal : fk_paciente_id
Paciente --> ConfiguracaoPaciente : fk_paciente_id
Paciente --> ConsentimentoPaciente : fk_paciente_id
ConfiguracaoPaciente --> CanalNotificacao
ConfiguracaoPaciente --> ModalidadePref
@enduml
```

## Domínio: Agendamento (`domain::agendamento`)

```plantuml
@startuml
skinparam classAttributeIconSize 0

enum ModalidadeConsulta <<enum>> {
    Presencial
    Online
}

enum StatusConsulta <<enum>> {
    Agendada
    Cancelada
    Realizada
}

class Consulta <<struct>> {
    + id: i32
    + fk_paciente_id: i32
    + fk_psicologo_id: i32
    + data_hora: NaiveDateTime
    + modalidade: ModalidadeConsulta
    + status: StatusConsulta
    + link_reuniao: Option<String>
}

class HorarioDisponivel <<struct>> {
    + id: i32
    + fk_psicologo_id: i32
    + dia_semana: i32
    + hora_inicio: NaiveTime
    + hora_fim: NaiveTime
}

Consulta --> ModalidadeConsulta
Consulta --> StatusConsulta
@enduml
```

## Serviços: Autenticação, Paciente e Agendamento (`services::*`)

```plantuml
@startuml
skinparam classAttributeIconSize 0

class AuthClaims <<struct>> {
    + sub: i32
    + exp: usize
}

enum PacienteError <<enum>> {
    DatabaseError(sqlx::Error)
    ValidationError(String)
}

class CadastroPacienteRequest <<struct>> {
    + fk_psicologo_id: i32
    + senha: Option<String>
    + nome: String
    + email: String
    + cpf: String
    + data_nascimento: NaiveDate
    + genero: Option<String>
    + estado_civil: Option<String>
    + telefone: Option<String>
    + telefone_alt: Option<String>
    + endereco: Option<String>
    + responsavel_nome: Option<String>
    ...
}

class PacienteResponse <<struct>> {
    + id: i32
    + nome: String
    + cpf: String
    + data_nascimento: NaiveDate
    + telefone: Option<String>
    + email: Option<String>
}

enum AgendamentoError <<enum>> {
    ConflitoHorario
    EpsiNaoValidado
    PsicologoNaoEncontrado
    DatabaseError(sqlx::Error)
}
@enduml
```
