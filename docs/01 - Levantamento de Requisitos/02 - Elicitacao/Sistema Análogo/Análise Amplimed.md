
# Especificação dos Requisitos do Sistema Análogo

#### Autores: Matheus Alves e Davi Coelho

## Identificação

- **Sistema análogo:** Amplimed
- **Categoria:** Sistema de gestão de consultórios e clínicas
- **Finalidade na pesquisa:** Servir de referência funcional para o levantamento de requisitos do sistema da clínica de psicologia

## Descrição Geral

O sistema análogo atende processos administrativos e clínicos compatíveis com o escopo do projeto, principalmente cadastro de pacientes, agendamento, confirmação de consulta, envio de lembretes, prontuário e controle de pagamento.

## Requisitos Funcionais

### RF-A01 — Cadastro de pacientes
- **Prioridade:** Essencial
- **Autor:** Matheus e Davi
- **Descrição:** O sistema análogo deve permitir cadastrar, editar e consultar os dados dos pacientes da clínica.

### RF-A02 — Agendamento de consultas
- **Prioridade:** Essencial
- **Autor:** Matheus e Davi
- **Descrição:** O sistema análogo deve permitir marcar consultas por profissional, data e horário.

### RF-A03 — Cancelamento e remarcação
- **Prioridade:** Essencial
- **Autor:** Matheus e Davi
- **Descrição:** O sistema análogo deve permitir cancelar ou remarcar consultas previamente registradas.

### RF-A04 — Confirmação de consulta
- **Prioridade:** Essencial
- **Autor:** Matheus e Davi
- **Descrição:** O sistema análogo deve permitir registrar a confirmação da consulta antes do atendimento.

### RF-A05 — Lembretes e avisos
- **Prioridade:** Desejável
- **Autor:** Matheus e Davi
- **Descrição:** O sistema análogo deve permitir o envio de lembretes de sessão e avisos relacionados a pagamentos ou consultas.

### RF-A06 — Prontuário do paciente
- **Prioridade:** Essencial
- **Autor:** Matheus e Davi
- **Descrição:** O sistema análogo deve permitir criar, atualizar e consultar prontuários vinculados ao paciente.

### RF-A07 — Controle de pagamento
- **Prioridade:** Essencial
- **Autor:** Matheus e Davi
- **Descrição:** O sistema análogo deve permitir registrar pagamentos, pendências e situação financeira das consultas.

### RF-A08 — Controle de usuários
- **Prioridade:** Essencial
- **Autor:** Matheus e Davi
- **Descrição:** O sistema análogo deve permitir autenticação e diferenciação de perfis de acesso, como recepcionista, psicólogo e administrador.

### RF-A09 — Consulta de agenda
- **Prioridade:** Essencial
- **Autor:** Matheus e Davi
- **Descrição:** O sistema análogo deve permitir visualizar a agenda por profissional e período.

### RF-A10 — Histórico operacional
- **Prioridade:** Desejável
- **Autor:** Matheus e Davi
- **Descrição:** O sistema análogo deve manter histórico de alterações em agendamentos, confirmações e pagamentos.

## Requisitos Não Funcionais

### RQ-A01 — Plataforma web
- **Prioridade:** Essencial
- **Autor:** Matheus e Davi
- **Descrição:** O sistema análogo deve operar em ambiente web.

### RQ-A02 — Compatibilidade futura com mobile
- **Prioridade:** Desejável
- **Autor:** Matheus e Davi
- **Descrição:** O sistema análogo deve possuir arquitetura que facilite futura adaptação para dispositivos móveis.

### RQ-A03 — Segurança da informação
- **Prioridade:** Essencial
- **Autor:** Matheus e Davi
- **Descrição:** O sistema análogo deve proteger dados pessoais e clínicos por autenticação, autorização e controle de acesso.

### RQ-A04 — Conformidade com LGPD
- **Prioridade:** Essencial
- **Autor:** Matheus e Davi
- **Descrição:** O sistema análogo deve tratar dados pessoais de forma compatível com a LGPD.

### RQ-A05 — Usabilidade
- **Prioridade:** Desejável
- **Autor:** Matheus e Davi
- **Descrição:** O sistema análogo deve oferecer interface simples e intuitiva para recepção e profissionais.

### RQ-A06 — Disponibilidade
- **Prioridade:** Desejável
- **Autor:** Matheus e Davi
- **Descrição:** O sistema análogo deve estar disponível durante o horário de funcionamento da clínica com baixo índice de falhas.

## Regras de Domínio

### RD-A01 — Acesso ao prontuário
- **Prioridade:** Essencial
- **Autor:** Matheus e Davi
- **Descrição:** Apenas profissionais autorizados podem visualizar ou editar prontuários.

### RD-A02 — Vínculo financeiro
- **Prioridade:** Essencial
- **Autor:** Matheus e Davi
- **Descrição:** Todo pagamento registrado deve estar associado a uma consulta ou atendimento.

### RD-A03 — Histórico de agenda
- **Prioridade:** Desejável
- **Autor:** Matheus e Davi
- **Descrição:** Alterações de status da consulta devem permanecer registradas para auditoria operacional.

### RD-A04 — Confirmação antes da sessão
- **Prioridade:** Desejável
- **Autor:** Matheus e Davi
- **Descrição:** O processo de atendimento pode depender da confirmação prévia da consulta, conforme regra da clínica.

## Restrições

### R-A01 — Domínio específico
- **Prioridade:** Essencial
- **Autor:** Matheus e Davi
- **Descrição:** O sistema análogo é voltado à gestão de consultórios e clínicas.

### R-A02 — Escopo adaptado ao projeto
- **Prioridade:** Essencial
- **Autor:** Matheus e Davi
- **Descrição:** Nem toda funcionalidade do sistema análogo precisa ser incorporada ao sistema da clínica de psicologia, apenas as compatíveis com o escopo definido pela disciplina.

### R-A03 — Base comparativa
- **Prioridade:** Essencial
- **Autor:** Matheus e Davi
- **Descrição:** O sistema análogo deve ser usado como referência de benchmark e não como definição final obrigatória do produto.

## Dados Principais

| ID | Descrição |
|---|---|
| DD-A01 | Dados do paciente: nome, contato e identificação cadastral |
| DD-A02 | Dados da consulta: data, hora, profissional, status e confirmação |
| DD-A03 | Dados do prontuário: registros clínicos, data e profissional responsável |
| DD-A04 | Dados financeiros: valor, vencimento, forma de pagamento e situação |
| DD-A05 | Dados de usuário do sistema: login, perfil e permissões |
| DD-A06 | Dados de comunicação: lembretes, avisos e status de envio |
