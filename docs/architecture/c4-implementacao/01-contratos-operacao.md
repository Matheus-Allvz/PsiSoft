# 3. Detalhamento de Implementação (Nível C4)

## 3.1 Contratos de Operação (Metodologia Larman)

Esta seção detalha os contratos de operação do sistema PsiSoft, elaborados de acordo com as diretrizes do Capítulo 11 da obra *Applying UML and Patterns* (Craig Larman). O objetivo destes contratos é descrever o comportamento do sistema de forma declarativa, focando estritamente nas mudanças de estado do Modelo de Domínio (instâncias criadas, associações formadas e atributos modificados), desvinculando o design da implementação algorítmica.

Nesta versão, as assinaturas das operações foram fortificadas utilizando a tipagem real da linguagem **Rust**, garantindo maior aderência ao código (ex: `i32`, `NaiveDateTime`, `String`, `bool`, `ModalidadeConsulta`). Conforme a padronização arquitetural exigida, todas as **Pós-condições** estão descritas no tempo verbal passado.

---

### Contrato de Operação 01: Cadastrar Paciente

* **Operação:** `cadastrar_paciente(pool: &PgPool, req: CadastroPacienteRequest) -> Result<i32, PacienteError>`
* **Referências Cruzadas:** 
  * Casos de Uso: Cadastro de Pacientes.
  * Módulos (C2/C3): API (`api::paciente`), Services (`services::paciente`), UI (`cadastro-paciente.html`).
* **Pré-condições:**
  * Deve existir uma instância de `Psicologo` cujo identificador (`id`) corresponda a `req.fk_psicologo_id`.
* **Pós-condições:**
  * Uma nova instância *u* de `Usuario` **foi criada**.
  * O atributo *u.perfil* **tornou-se** `'paciente'`.
  * O atributo *u.senha_hash* **tornou-se** o hash criptográfico gerado a partir de `req.senha` (ou `'123456'` caso omitida).
  * O atributo *u.login* **tornou-se** o valor de `req.cpf` (se não vazio) ou o valor de `req.email`.
  * Uma nova instância *p* de `Paciente` **foi criada**.
  * Uma associação **foi formada** entre a instância *p* e a instância *u* (`fk_usuario_id`).
  * Uma associação **foi formada** entre a instância *p* e a instância existente de `Psicologo` (`req.fk_psicologo_id`).
  * Se `req.responsavel_nome` foi fornecido, uma nova instância *r* de `ResponsavelLegal` **foi criada** e uma associação **foi formada** entre *r* e *p*.
  * Uma nova instância *c* de `ConfiguracaoPaciente` **foi criada** com base em `req.config_canal` e `req.config_modalidade` e associada a *p*.
  * Uma nova instância *co* de `ConsentimentoPaciente` **foi criada** e associada a *p*, com o atributo `data_hora_aceite` definido para o momento exato da transação.
  * O disparo assíncrono de um e-mail de boas-vindas contendo as credenciais **foi agendado** (Spawn Tokio).

---

### Contrato de Operação 02: Marcar Agendamento

* **Operação:** `criar_agendamento(pool: &PgPool, fk_paciente_id: i32, fk_psicologo_id: i32, data_hora: NaiveDateTime, modalidade: ModalidadeConsulta) -> Result<Consulta, AgendamentoError>`
* **Referências Cruzadas:** 
  * Casos de Uso: Agendamento de Consultas.
  * Módulos (C2/C3): API (`api::agendamento`), Services (`services::agendamento`), UI (`agendamentos.html`).
* **Pré-condições:**
  * Deve existir uma instância de `Paciente` com `id` igual a `fk_paciente_id`.
  * Deve existir uma instância de `Psicologo` com `fk_usuario_id` igual a `fk_psicologo_id`.
  * Se o parâmetro `modalidade` for `ModalidadeConsulta::Online`, o atributo `validador_epsi` da instância de `Psicologo` encontrada não pode estar nulo (Deve ter sido validado para consultas online).
  * A agenda do `Psicologo` não deve possuir nenhuma instância de `Consulta` na mesma `data_hora` cujo atributo `status` seja diferente de `'Cancelada'` (Garantia de ausência de conflito de horário).
* **Pós-condições:**
  * Uma nova instância *c* de `Consulta` **foi criada**.
  * O atributo *c.data_hora* **tornou-se** o valor recebido no parâmetro `data_hora`.
  * O atributo *c.modalidade* **tornou-se** o valor recebido no parâmetro `modalidade`.
  * O atributo *c.status* **tornou-se** `StatusConsulta::Agendada`.
  * Uma associação (Chave Estrangeira do BD) **foi formada** entre a instância *c* e a instância referenciada de `Psicologo` (`fk_psicologo_id`).
  * Uma associação **foi formada** entre a instância *c* e a instância referenciada de `Paciente` (`fk_paciente_id`).
* **Regras de Negócio Associadas:** 
  * O sistema impede agendamentos simultâneos e valida obrigatoriamente a certificação E-PSI do Conselho de Psicologia para atendimentos na modalidade online.