# Síntese dos Requisitos dos Questionários de Psicólogo

**Versão:** 1.0
**Data de Emissão:** 04/04/2026
**Responsável:** Daniel Carvalho Coutinho
**Base de Referência:** Levantamento de Campo (Questionário-Psicóloga ), ISO 25010, ISO 27001, LGPD e Resoluções do CFP.

---

## 1. Transcrição Geral

Este documento apresenta o levantamento de requisitos que visa abordar e transcrever as reais necessidades do psicólogo frente ao seu trabalho diário e ao gerenciamento da sua prática clínica.

## 2. Restrições do Sistema e do Negócio

- **REST-01 (Orçamentária):** O custo de infraestrutura em nuvem e manutenção do sistema deve ser mantido ao mínimo para viabilizar um modelo de assinatura de baixo custo para o usuário final.
- **REST-02 (Regulatória - CFP):** O sistema deve garantir a guarda obrigatória dos prontuários eletrônicos pelo período mínimo de 5 (cinco) anos, conforme exigência do Conselho Federal de Psicologia, assegurando que não haja risco de perda de dados.
- **REST-03 (Regulatória - LGPD):** Por tratar dados de saúde (dados sensíveis, Art. 5º, II da LGPD), o banco de dados e as transmissões de rede devem possuir criptografia de ponta a ponta.
- **REST-04 (Operacional):** A transição do papel para o digital gera alta resistência se o sistema for complexo; portanto, o número de cliques para o registro diário de prontuários deve ser rigidamente minimizado.

---

## 3. Classificação de Prioridades (Matriz MoSCoW)

- **Must Have (Obrigatório):** Crítico para o lançamento (MVP). O sistema não funciona sem isso.
- **Should Have (Importante):** Alto valor, mas não impede o lançamento básico.
- **Could Have (Desejável):** Bom ter, mas será implementado apenas se houver tempo/recursos.
- **Won't Have (Não terá agora):** Fora do escopo para esta fase.

---

## 4. Requisitos Funcionais (RF)

| ID        | Nome do Requisito                      | Descrição                                                                                                                                                                                         | Prioridade | Origem   |
| :-------- | :------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :--------- | :------- |
| **RF-01** | **Prontuário Clínico Híbrido**         | O sistema deve prover um módulo de prontuário que combine campos orientadores básicos (ID da sessão, queixa) com amplos espaços discursivos (rich text) para registro narrativo fluido.           | **Must**   | Q05, Q13 |
| **RF-02** | **Exportação Integral de Dados**       | O sistema deve permitir que o profissional exporte todo o histórico clínico de um paciente (com autorização) em um formato legível (ex: PDF estruturado) com um único clique.                     | **Must**   | Q12      |
| **RF-03** | **Gestão de Agenda e Políticas**       | O sistema deve permitir o agendamento de sessões com parametrização manual de políticas de faltas e remarcações, permitindo ao profissional aplicar flexibilidade ou regras rígidas por paciente. | **Should** | Q03      |
| **RF-04** | **Dashboard Financeiro Líquido**       | O módulo financeiro deve calcular automaticamente e exibir a separação entre "faturamento bruto" e "lucro real", deduzindo despesas e taxas de transação.                                         | **Should** | Q04      |
| **RF-05** | **Linha do Tempo Visual (Timeline)**   | O sistema deve gerar uma representação gráfica cronológica destacando os principais marcos e a evolução do tratamento do paciente.                                                                | **Could**  | Q07      |
| **RF-06** | **Alertas de Inatividade Terapêutica** | O sistema deve notificar ativamente o profissional caso um paciente fique sem agendar sessões por um período superior ao configurado (ex: 15 dias, 30 dias).                                      | **Could**  | Q14      |
| **RF-07** | **Controle Manual de Lembretes**       | O sistema deve permitir que o envio automático de lembretes seja desativado, caso o profissional prefira realizar o aviso manualmente para manter o vínculo.                                      | **Could**  | Q02      |

---

## 5. Requisitos Não Funcionais (RNF)

Esta seção mapeia os atributos de qualidade baseados nas normas da International Organization for Standardization (ISO).

### 5.1. Qualidade e Usabilidade (Baseado na ISO/IEC 25010)

| ID         | Atributo                             | Descrição                                                                                                                                       | Prioridade | Origem |
| :--------- | :----------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------- | :--------- | :----- |
| **RNF-01** | **Operabilidade / Inteligibilidade** | A interface do prontuário deve ser projetada para exigir baixo esforço cognitivo, minimizando a resistência à transcrição de anotações físicas. | **Must**   | Q06    |
| **RNF-02** | **Portabilidade / Responsividade**   | O sistema deve ser integralmente funcional e visualmente otimizado para acesso via dispositivos móveis (smartphones e tablets).                 | **Must**   | Q08    |

### 5.2. Segurança da Informação (Baseado na ISO/IEC 27001)

| ID         | Atributo                           | Descrição                                                                                                                                                                                     | Prioridade | Origem |
| :--------- | :--------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------- | :----- |
| **RNF-03** | **Confidencialidade**              | Garantia de que as informações clínicas não comprometerão a qualidade do registro nem sofrerão vazamentos de qualquer natureza.                                                               | **Must**   | Q09    |
| **RNF-04** | **Controle de Acesso / Agilidade** | O sistema deve suportar métodos de autenticação ágeis e seguros em dispositivos móveis (ex: Biometria, FaceID, TouchID ou PIN curto de 4 a 6 dígitos), evitando travamentos na rotina diária. | **Should** | Q10    |
| **RNF-05** | **Disponibilidade / Backup**       | A arquitetura deve prever rotinas automáticas de backup para garantir a recuperação rápida de dados e o cumprimento da guarda documental sem risco de perda.                                  | **Must**   | Q11    |

---

## 6. Regras de Negócio (RN)

- **RN-01 (Propriedade dos Dados):** O prontuário pertence ao paciente, e sua guarda é responsabilidade do profissional. O sistema atua apenas como operador dos dados.
- **RN-02 (Consentimento de Exportação):** A exportação de dados para necessidades externas deve ocorrer estritamente mediante o aval/solicitação do paciente titular.
- **RN-03 (Identidade da Psicologia):** Nenhuma funcionalidade médica engessada (como emissão de CID obrigatória para fechar tela ou prescrições medicamentosas) deve ser exigida no fluxo principal de atendimento psicológico.
