# Especificação de Arquitetura — PsiSoft

## 1. Introdução

### 1.1 Propósito do Documento

Este documento descreve a arquitetura de software do **PsiSoft**, uma plataforma SaaS (Software as a Service) de gestão clínica, administrativa e financeira voltada a profissionais de saúde mental no Brasil. O sistema tem como objetivos centrais reduzir a carga operacional dos profissionais, digitalizar o fluxo do consultório e garantir conformidade com as exigências legais e éticas do setor (LGPD, Código de Ética do CFP e Resolução CFP n° 001/2009).

### 1.2 Estrutura do Documento

A especificação está organizada seguindo a simplificação do modelo ISO/IEC/IEEE 42010:2022, com progressão em quatro níveis de abstração:

| Nível | Descrição |
|-------|-----------|
| **C1 — Contexto (Alto Nível)** | Infraestrutura de implantação, atores externos, fluxos de negócio e fronteiras do sistema. Inclui Diagrama de Implantação e Diagramas de Atividade (BPMN). |
| **C2 — Design Detalhado** | Particionamento do sistema em pacotes de negócio e técnicos. Inclui Diagrama de Pacotes e descrição dos Casos de Uso. |
| **C3 — Componentes e Dados** | Estrutura interna de cada pacote. Inclui Diagramas de Componentes, Modelo Entidade-Relacionamento e Dicionário de Dados. |
| **C4 — Implementação** | Detalhamento de código e banco de dados. Inclui Esquema Operacional de BD, Diagramas de Classes, Diagramas de Sequência e Contratos de Operação. |

---

## 2. Stakeholders

Os stakeholders foram identificados a partir das entrevistas e questionários conduzidos na fase de elicitação de requisitos. Para cada grupo são descritos seus interesses e as preocupações arquiteturais que os afetam diretamente.

### 2.1 Psicólogo / Profissional de Saúde Mental

**Perfil:** Usuário primário do sistema. Utiliza a plataforma no dia a dia do consultório para gestão clínica, administrativa e financeira.

**Interesses:**
- Registro ágil e estruturado de prontuários eletrônicos, com o mínimo de cliques possível.
- Agenda centralizada com controle de horários, cancelamentos e confirmações automáticas.
- Transcrição automática de sessões via Inteligência Artificial para redução de carga manual.
- Emissão de documentos fiscais (recibos e laudos) diretamente pela plataforma.
- Controle de acesso granular: separação estrita entre informações clínicas (exclusivas do profissional) e informações administrativas (acessíveis à secretária).
- Conformidade nativa com a Resolução CFP n° 001/2009 e com o Código de Ética do CFP.

**Preocupações arquiteturais:** Usabilidade (RNF-04), conformidade legal (RNF-07), criptografia (RNF-01) e disponibilidade (RNF-03).

---

### 2.2 Paciente

**Perfil:** Usuário final do portal do paciente. Interage com o sistema para agendamentos, acesso a documentos e comunicação com o profissional.

**Interesses:**
- Agendamento autônomo de consultas sem necessidade de ligação ou intermediação humana.
- Cancelamento e remarcação ágil com visualização imediata de horários disponíveis.
- Recebimento de lembretes de sessão via WhatsApp com 24 horas de antecedência.
- Acesso e download de recibos, laudos e comprovantes de pagamento.
- Garantia de privacidade e proteção de dados pessoais (LGPD).
- Interface responsiva e funcional em dispositivos móveis.

**Preocupações arquiteturais:** Responsividade (RNF-05), segurança de dados (RNF-01, RNF-02) e usabilidade (RNF-04).

---

### 2.3 Secretária / Recepcionista

**Perfil:** Usuário operacional com acesso restrito ao módulo administrativo. Realiza agendamentos e controle de presença, sem acesso a prontuários clínicos.

**Interesses:**
- Visualização e gestão da agenda do profissional.
- Confirmação de consultas e registro de presença.
- Controle financeiro básico (recebimentos e inadimplências).

**Preocupações arquiteturais:** Controle de acesso por perfil (RNF-01, RNF-02), separação entre módulo clínico e administrativo.

---

### 2.4 Contador

**Perfil:** Usuário externo com acesso pontual a informações financeiras da clínica.

**Interesses:**
- Acesso a relatórios financeiros e histórico de pagamentos.
- Exportação de dados para fins contábeis e fiscais.

**Preocupações arquiteturais:** Controle de acesso restrito, integridade dos dados financeiros.

---

### 2.5 Conselho Federal de Psicologia (CFP)

**Perfil:** Stakeholder regulatório. Não utiliza o sistema diretamente, mas impõe restrições normativas que moldam toda a arquitetura.

**Interesses:**
- Conformidade dos prontuários com a Resolução CFP n° 001/2009 (campos obrigatórios, prazo de guarda de 5 anos).
- Validação do Cadastro e-Psi para atendimentos realizados por meios tecnológicos (Resolução CFP n° 011/2018).
- Sigilo ético na comunicação com pacientes.

**Preocupações arquiteturais:** Conformidade (RNF-07), auditabilidade, controle de exclusão de registros (RN-01).

---

### 2.6 Autoridade Nacional de Proteção de Dados (ANPD) / LGPD

**Perfil:** Stakeholder regulatório. Define os requisitos de privacidade e proteção de dados pessoais sensíveis (dados de saúde).

**Interesses:**
- Criptografia de dados em repouso e em trânsito.
- Gestão de consentimento com registro de aceite (TCLE).
- Direito de acesso e retificação de dados pelo titular.
- Impedimento de exclusão dentro do prazo legal de guarda.

**Preocupações arquiteturais:** Segurança (RNF-01), rastreabilidade de consentimento (RF-03), direitos do titular (RF-02, RF-10).

---

### 2.7 Equipe de Desenvolvimento

**Perfil:** Desenvolvedores e arquitetos responsáveis pela construção e manutenção do sistema.

**Interesses:**
- Arquitetura modular e de fácil manutenção.
- Separação clara de responsabilidades entre camadas (API, Services, Domain, Events).
- Infraestrutura de custo operacional baixo para viabilizar modelo SaaS de assinatura acessível (REST-01).
- Documentação arquitetural rastreável e atualizada.

**Preocupações arquiteturais:** Manutenibilidade, escalabilidade, custo de infraestrutura.

---

## 3. Requisitos Não Funcionais e seu Tratamento na Arquitetura

Esta seção descreve os principais Requisitos Não Funcionais (RNFs) do PsiSoft e como cada um é endereçado pelas decisões arquiteturais adotadas.

### 3.1 Segurança e Privacidade (RNF-01, RNF-02)

**Requisito:** Todas as comunicações e dados armazenados devem ser criptografados. O acesso deve exigir autenticação multifator (MFA).

**Tratamento na arquitetura:**
- Toda comunicação entre o cliente (frontend PWA/App) e a API backend ocorre exclusivamente via **HTTPS/TLS 1.3**, eliminando tráfego em texto claro.
- Os dados armazenados no banco de dados PostgreSQL são protegidos por **criptografia AES-256 em repouso**.
- A instância de banco de dados é isolada em uma **rede privada (VPC)**, sem exposição direta à internet; o acesso ocorre apenas internamente via TCP/IP.
- O módulo de autenticação (`api::auth`) implementa MFA com suporte a biometria e PIN para acesso mobile.
- O controle de acesso por perfil (psicólogo, secretária, paciente, contador) é gerenciado no `domain::user`, garantindo que dados clínicos sejam acessíveis apenas ao profissional responsável.

---

### 3.2 Disponibilidade e Integridade dos Dados (RNF-03)

**Requisito:** O sistema deve garantir alta disponibilidade e rotinas automáticas de backup para evitar perda de registros clínicos obrigatórios.

**Tratamento na arquitetura:**
- A infraestrutura é hospedada em ambiente de nuvem (AWS/GCP) com instâncias separadas para o servidor de aplicação e o servidor de banco de dados, permitindo políticas de replicação e backup independentes.
- A regra de negócio RN-01 (prazo mínimo de guarda de 5 anos) é imposta no nível da camada de domínio (`domain::`), impedindo exclusões definitivas de prontuários dentro do período legal.
- A arquitetura de backend em Rust (Axum + Tokio + SQLx) oferece alta performance e confiabilidade com controle explícito de concorrência assíncrona, reduzindo riscos de falhas em picos de carga.

---

### 3.3 Usabilidade e Responsividade (RNF-04, RNF-05)

**Requisito:** A interface deve ser intuitiva, com o mínimo de cliques para operações recorrentes (ex.: registro de prontuário), e totalmente funcional em dispositivos móveis.

**Tratamento na arquitetura:**
- O frontend é desenvolvido como **Progressive Web App (PWA)**, garantindo funcionamento nativo em smartphones e tablets sem necessidade de app store, reduzindo fricção de adoção.
- O design de interface adota o padrão de ações rápidas diretamente nas telas de agenda e prontuário, com foco na redução de navegação para tarefas de alta frequência.
- A arquitetura de API REST do backend expõe endpoints coesos para cada caso de uso, evitando múltiplas chamadas para operações simples.

---

### 3.4 Conformidade com o CFP (RNF-07)

**Requisito:** A estrutura do prontuário eletrônico deve seguir as informações obrigatórias definidas pela Resolução CFP n° 001/2009.

**Tratamento na arquitetura:**
- O módulo de prontuário (`domain::prontuario`) é modelado diretamente sobre os campos obrigatórios da Resolução: identificação do paciente, queixa principal, procedimentos técnicos adotados, evolução do caso e encaminhamentos.
- O sistema impede o encerramento de um registro de sessão sem o preenchimento dos campos obrigatórios, garantindo conformidade por design.
- A integração com o **Cadastro e-Psi** (via API CFP por HTTPS/REST) valida o registro do profissional antes de habilitar funcionalidades de atendimento online, conforme exigido pela Resolução CFP n° 011/2018.

---

### 3.5 Transcrição com IA (RNF-06)

**Requisito:** O sistema deve transcrever automaticamente o conteúdo de sessões e gerar rascunhos de prontuário.

**Tratamento na arquitetura:**
- A funcionalidade de transcrição é delegada a um **Motor de IA externo** (integrado via HTTPS/REST), mantendo a lógica de IA fora do backend principal e evitando acoplamento de alta complexidade no núcleo do sistema.
- O resultado da transcrição retorna à camada de serviço (`api::sessao`) como rascunho editável, exigindo revisão e confirmação do profissional antes da gravação definitiva no prontuário, conforme exigido pelo RF-L06 (supervisão humana obrigatória).
- Dados de áudio e transcrição **não são armazenados permanentemente**; apenas o texto aprovado pelo profissional é persistido no banco de dados criptografado.
