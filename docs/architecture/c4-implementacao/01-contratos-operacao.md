# 3. Detalhamento de Implementação (Nível C4)

## 3.1 Contratos de Operação (Metodologia Larman)

Esta seção detalha os contratos de operação do sistema PsiSoft, elaborados de acordo com as diretrizes do Capítulo 11 da obra *Applying UML and Patterns* (Craig Larman). O objetivo destes contratos é descrever o comportamento do sistema de forma declarativa, focando estritamente nas mudanças de estado do Modelo de Domínio (instâncias criadas, associações formadas e atributos modificados), desvinculando o design da implementação algorítmica.

Nesta versão, as assinaturas das operações foram fortificadas utilizando a tipagem real da linguagem **Rust**, garantindo maior aderência ao código (ex: `uuid::Uuid`, `chrono::DateTime<Utc>`, `String`, `bool`, `rust_decimal::Decimal`). Conforme a padronização arquitetural exigida, todas as **Pós-condições** estão descritas no tempo verbal passado.

---

### Contrato de Operação 01: Autenticação de Usuário (Login)

* **Operação:** `authenticate(email: String, password: String) -> Result<String, AuthError>`
* **Referências Cruzadas:** 
  * Casos de Uso: Login na Plataforma.
  * Módulos (C2/C3): API, Services (Auth), Domain (User).
* **Pré-condições:**
  * Deve existir uma instância de `User` no banco de dados (`Sqlx`) cujo atributo `email` corresponda exatamente ao parâmetro `email`.
  * A senha fornecida em `password`, ao passar pela função de verificação criptográfica (Argon2), deve coincidir com o `password_hash` da instância de `User` encontrada.
* **Pós-condições:**
  * Uma nova sessão virtual de usuário (representada criptograficamente por um Token JWT) **foi gerada**.
  * O atributo `last_login_at` (do tipo `chrono::DateTime<Utc>`) da instância de `User` **foi alterado** para o timestamp atual do servidor.
* **Regras de Negócio Associadas:** 
  * Senhas nunca devem ser trafegadas ou armazenadas em texto plano.
  * Em caso de credenciais inválidas, a operação deve retornar um erro, e o estado do sistema não sofre alterações.

---

### Contrato de Operação 02: Registro Inicial de Agendamento

* **Operação:** `criar_agendamento(id_psicologo: uuid::Uuid, id_paciente: uuid::Uuid, data_hora: chrono::DateTime<Utc>, modalidade: String) -> Result<uuid::Uuid, AgendamentoError>`
* **Referências Cruzadas:** 
  * Casos de Uso: Agendamento Autônomo.
  * Módulos (C2/C3): API, Services (Agendamento), Domain (Consulta).
* **Pré-condições:**
  * Deve existir uma instância de `Psicologo` cujo identificador seja `id_psicologo` (tipo `uuid::Uuid`) com status ativo.
  * Deve existir uma instância de `Paciente` cujo identificador seja `id_paciente` (tipo `uuid::Uuid`).
  * A agenda do `Psicologo` não deve possuir nenhuma instância de `Agendamento` na mesma `data_hora` cujo atributo `status` (tipo `String` ou enum) seja diferente de `'Cancelado'`.
* **Pós-condições:**
  * Uma nova instância *a* de `Agendamento` **foi criada**.
  * O atributo *a.id* **tornou-se** um novo identificador único (`uuid::Uuid`).
  * O atributo *a.data_hora* **tornou-se** o valor recebido no parâmetro `data_hora`.
  * O atributo *a.status* **tornou-se** a string `'Pendente'`.
  * O atributo *a.modalidade* **tornou-se** o valor recebido no parâmetro `modalidade`.
  * Uma associação (Chave Estrangeira do BD) **foi formada** entre a instância *a* e a instância existente de `Psicologo`.
  * Uma associação **foi formada** entre a instância *a* e a instância existente de `Paciente`.
* **Regras de Negócio Associadas:** 
  * O agendamento inicial é sempre criado como bloqueado e aguarda a finalização e aprovação do gateway de pagamento para efetivação definitiva.

---

### Contrato de Operação 03: Efetivação de Agendamento (Pagamento Webhook)

* **Operação:** `confirmar_pagamento(id_agendamento: uuid::Uuid, id_transacao: String, valor_pago: rust_decimal::Decimal) -> Result<(), PagamentoError>`
* **Referências Cruzadas:** 
  * Casos de Uso: Confirmação de Pagamento, Notificações.
  * Módulos (C2/C3): API, Services, Domain, Events.
* **Pré-condições:**
  * Deve existir uma instância de `Agendamento` correspondente a `id_agendamento` e que possua o status atual `'Pendente'`.
  * A assinatura criptográfica do Webhook do gateway (no header HTTP validado pelo Axum) deve ser autêntica.
* **Pós-condições:**
  * Uma nova instância *p* de `Pagamento` **foi criada**.
  * O atributo *p.id_transacao* **tornou-se** o valor do parâmetro `id_transacao`.
  * O atributo *p.valor* **tornou-se** o valor recebido em `valor_pago`.
  * O atributo *p.quitado* (tipo `bool`) **tornou-se** `true`.
  * Uma associação **foi formada** entre a instância *p* (`Pagamento`) e a instância referenciada de `Agendamento`.
  * O atributo *status* da instância associada de `Agendamento` **foi alterado** de `'Pendente'` para `'Confirmado'`.
* **Eventos Disparados:**
  * A confirmação do agendamento aciona a camada `Events` (Tokio) para o disparo assíncrono de notificações via WhatsApp (API da Meta) e atualização SSE na UI.