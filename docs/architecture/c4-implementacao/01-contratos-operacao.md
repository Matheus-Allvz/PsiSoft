# 3. Detalhamento de Implementação (Nível C4)

## 3.1 Contratos de Operação (Metodologia Larman)

Esta seção detalha os contratos de operação do sistema PsiSoft, elaborados de acordo com as diretrizes do Capítulo 11 da obra *Applying UML and Patterns* (Craig Larman). O objetivo destes contratos é descrever o comportamento do sistema de forma declarativa, focando estritamente nas mudanças de estado do Modelo de Domínio (instâncias criadas, associações formadas e atributos modificados), desvinculando o design da implementação algorítmica.

Conforme a padronização arquitetural exigida, todas as **Pós-condições** estão descritas no tempo verbal passado.

---

### Contrato de Operação 01: Registro Inicial de Reserva

* **Operação:** `registrarConsultaPendente(idPsicologo: INT, idPaciente: INT, dataHora: DATETIME, modalidade: ENUM)`
* **Referências Cruzadas:** 
  * Casos de Uso: Agendamento Autônomo.
  * Módulos (C2/C3): Agendamento, Consulta.
* **Pré-condições:**
  * Deve existir uma instância de `Psicologo` cujo identificador seja `idPsicologo` e que possua status ativo na plataforma.
  * Deve existir uma instância de `Paciente` cujo identificador seja `idPaciente`.
  * Não deve existir nenhuma instância de `Consulta` associada a este `idPsicologo` onde o atributo `data_hora` seja igual a `dataHora` e o atributo `presenca` seja diferente de 'Cancelada' (Garantia contra *double-booking*).
* **Pós-condições:**
  * Uma nova instância *c* de `Consulta` **foi criada**.
  * O atributo *c.data_hora* **tornou-se** o valor do parâmetro `dataHora`.
  * O atributo *c.presenca* **tornou-se** 'Pendente' (estado transitório aguardando gateway).
  * O atributo *c.modalidade* **tornou-se** o valor do parâmetro `modalidade` ('Presencial' ou 'Online').
  * Uma associação **foi formada** entre a instância *c* (`Consulta`) e a instância existente de `Psicologo` correspondente a `idPsicologo`.
  * Uma associação **foi formada** entre a instância *c* (`Consulta`) e a instância existente de `Paciente` correspondente a `idPaciente`.
* **Regras de Negócio Associadas:** 
  * A consulta nasce bloqueada temporariamente para outros pacientes, mas sua efetivação depende estritamente do retorno positivo do fluxo financeiro em até 15 minutos (timeout).

---

### Contrato de Operação 02: Geração de Ordem de Cobrança

* **Operação:** `gerarOrdemCobranca(idConsulta: INT, valorSessao: DECIMAL(10,2))`
* **Referências Cruzadas:** 
  * Casos de Uso: Agendamento Autônomo, Processamento de Pagamento.
  * Módulos (C2/C3): Consulta, Gestão Financeira.
* **Pré-condições:**
  * Deve existir uma instância de `Consulta` cujo identificador seja `idConsulta`.
  * O atributo `presenca` da instância de `Consulta` deve ser exatamente igual a 'Pendente'.
  * Não deve existir uma instância prévia de `Pagamento` já associada a esta `Consulta` com o atributo `quitado` igual a `True`.
* **Pós-condições:**
  * Uma nova instância *p* de `Pagamento` **foi criada**.
  * O atributo *p.valor* **tornou-se** o valor do parâmetro `valorSessao`.
  * O atributo *p.quitado* **tornou-se** `False` (tipo BOOLEAN).
  * O atributo *p.metodo* **tornou-se** `NULL` (aguardando a escolha e retorno da API externa do Gateway).
  * Uma associação **foi formada** entre a instância *p* (`Pagamento`) e a instância de `Consulta` correspondente a `idConsulta`.
* **Notas Arquiteturais:** 
  * Esta operação precede imediatamente o disparo da requisição HTTP (REST) para o Gateway de Pagamento. A entidade `Pagamento` local servirá de âncora para o Webhook futuro.

---

### Contrato de Operação 03: Efetivação Reativa via Webhook

* **Operação:** `efetivarConsulta(idConsulta: INT, idPagamento: INT, metodoGateway: VARCHAR)`
* **Referências Cruzadas:** 
  * Casos de Uso: Confirmação de Agendamento, Notificação Automática.
  * Módulos (C2/C3): Gestão Financeira, Consulta, Notificação.
* **Pré-condições:**
  * Deve existir uma instância de `Consulta` correspondente a `idConsulta` com status 'Pendente'.
  * Deve existir uma instância de `Pagamento` correspondente a `idPagamento` associada a esta consulta.
  * O atributo `quitado` desta instância de `Pagamento` deve ser `False` antes da execução.
  * A assinatura criptográfica do Webhook (header) deve ter sido validada (RNF-01 de Segurança).
* **Pós-condições:**
  * O atributo *quitado* da instância existente de `Pagamento` **foi alterado** para `True`.
  * O atributo *metodo* da instância existente de `Pagamento` **foi alterado** para o valor recebido em `metodoGateway` (ex: 'Pix', 'Cartao').
  * O atributo *presenca* da instância de `Consulta` **foi alterado** de 'Pendente' para 'Agendada' no banco de dados.
  * Uma nova instância *n* de `Notificacao` **foi criada**.
  * O atributo *n.conteudo_msg* **foi preenchido** com a string de confirmação contendo os dados do paciente e data.
  * Uma associação **foi formada** entre a instância *n* (`Notificacao`) e a instância de `Consulta`.
* **Eventos Disparados:**
  * O componente de Notificação empurra o alerta para o WhatsApp (API Meta).
  * O WebSocket é acionado para atualizar a Interface do Usuário (UI) do Paciente.