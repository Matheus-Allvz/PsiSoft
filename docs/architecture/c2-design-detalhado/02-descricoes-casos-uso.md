## 2. Descrições de Casos de Uso (Formato Expandido - Larman)

### Caso de Uso: Login
**Ator Principal:** Psicólogo / Paciente  
**Pré-condições:** O ator deve estar registrado no sistema com credenciais válidas.  
**Garantias de Sucesso (Pós-condições):** O sistema gera e retorna um token JWT, além de despachar um evento de sucesso de login.

**Cenário de Sucesso Principal (Fluxo Básico):**
1. O Ator insere suas credenciais (email/login e senha) e solicita o login.
2. O Sistema busca o usuário no banco de dados através do email ou login.
3. O Sistema verifica se a senha fornecida corresponde ao hash armazenado usando o algoritmo Argon2.
4. O Sistema gera um token JWT associado ao ID do usuário.
5. O Sistema busca o CRP do psicólogo (se aplicável).
6. O Sistema emite um evento interno de `LoginSuccess` no EventBus.
7. O Sistema retorna o token JWT, ID do usuário, nome e CRP (se existir).

**Extensões (Fluxos Alternativos):**
* 2a. Usuário não encontrado no banco de dados.
  1. O Sistema retorna um erro de credenciais inválidas (HTTP 401).
* 3a. A senha fornecida está incorreta.
  1. O Sistema retorna um erro de credenciais inválidas (HTTP 401).

---

### Caso de Uso: Cadastro de Paciente
**Ator Principal:** Psicólogo  
**Pré-condições:** O Psicólogo deve estar autenticado e autorizado no sistema.  
**Garantias de Sucesso (Pós-condições):** O Paciente é persistido no banco de dados com usuário correspondente, configurações, consentimentos e (opcionalmente) responsável legal. Um e-mail de boas-vindas é enviado ao paciente.

**Cenário de Sucesso Principal (Fluxo Básico):**
1. O Psicólogo fornece os dados do paciente (nome, CPF, data de nascimento, contatos, etc.) e envia a requisição de cadastro.
2. O Sistema inicia uma transação no banco de dados.
3. O Sistema define a senha (fornecida ou padrão "123456") e gera seu hash.
4. O Sistema cria um registro de `Usuario` (perfil: paciente).
5. O Sistema cria um registro de `Paciente` vinculado ao `Usuario` recém-criado e ao ID do Psicólogo.
6. O Sistema salva as preferências de notificação (`ConfiguracaoPaciente`).
7. O Sistema salva os dados de consentimento e LGPD (`ConsentimentoPaciente`).
8. O Sistema efetiva a transação (commit).
9. O Sistema inicia um processo assíncrono (Task Tokio) para enviar o e-mail de boas-vindas.
10. O Sistema retorna o ID do novo paciente criado (HTTP 201).

**Extensões (Fluxos Alternativos):**
* 5a. O paciente possui um Responsável Legal informado.
  1. O Sistema insere os dados do responsável na tabela `ResponsavelLegal`, vinculado ao paciente, antes de prosseguir com as configurações.
* 2a - 8a. Ocorrer uma falha no banco de dados em qualquer etapa da transação.
  1. O Sistema realiza um *rollback* da transação.
  2. O Sistema retorna um erro de banco de dados (HTTP 500).

---

### Caso de Uso: Agendamento de Consulta
**Ator Principal:** Paciente / Psicólogo  
**Pré-condições:** O Psicólogo e o Paciente devem existir no sistema.  
**Garantias de Sucesso (Pós-condições):** Uma nova consulta é registrada no sistema com status "Agendada".

**Cenário de Sucesso Principal (Fluxo Básico):**
1. O Ator solicita o agendamento fornecendo ID do paciente, ID do psicólogo, data/hora e modalidade (Presencial/Online).
2. O Sistema verifica se existe conflito de horário (outra consulta não cancelada para o mesmo psicólogo no mesmo horário).
3. O Sistema registra a nova `Consulta` no banco de dados com o status `Agendada`.
4. O Sistema retorna os detalhes da consulta recém-criada (HTTP 201).

**Extensões (Fluxos Alternativos):**
* 1a. A modalidade escolhida é `Online`.
  1. O Sistema busca o cadastro do Psicólogo.
  2. O Sistema verifica se o Psicólogo possui validação no e-Psi (`validador_epsi` não é nulo).
  3. O Sistema prossegue para o passo 2.
* 1a2. O Psicólogo não possui validação no e-Psi.
  1. O Sistema bloqueia o agendamento e retorna o erro "Psicólogo não validado para consultas online".
* 2a. O horário selecionado já está ocupado por outra consulta.
  1. O Sistema retorna o erro "Conflito de horário para este psicólogo".
