# Síntese dos Requisitos dos Questionários de Paciente

## Identificação

- **Sistema:** PsiSoft
- **Categoria:** Sistema de gestão de clínica e portal do paciente
- **Finalidade na pesquisa:** Especificar requisitos focados nas dores, necessidades e expectativas dos pacientes a partir dos questionários aplicados.

## Descrição Geral

O levantamento traduz as necessidades de pacientes reais para o portal PsiSoft, focando na autonomia do agendamento, comunicação automatizada, segurança no manuseio de dados sensíveis (LGPD) e controle financeiro integrado.

## Requisitos Funcionais

### RF-P01 — Agendamento autônomo

- **Prioridade:** Essencial
- **Descrição:** O sistema deve permitir que o paciente acesse a agenda digital do profissional e marque consultas de forma autônoma, sem intervenção humana.

### RF-P02 — Cancelamento e remarcação ágil

- **Prioridade:** Essencial
- **Descrição:** O sistema deve permitir o cancelamento de consultas e, em caso de remarcação, apresentar imediatamente os dias e horários disponíveis.

### RF-P03 — Emissão de recibos e comprovantes

- **Prioridade:** Essencial
- **Descrição:** O sistema deve possuir uma aba exclusiva e organizada para emissão de recibos (para Imposto de Renda) e comprovantes, com datas e histórico de consultas passadas.

### RF-P04 — Lembretes de sessão via WhatsApp

- **Prioridade:** Essencial
- **Descrição:** O sistema deve enviar lembretes de consulta com 24 horas de antecedência através do WhatsApp, utilizando tom profissional e acolhedor.

### RF-P05 — Ações rápidas na notificação

- **Prioridade:** Desejável
- **Descrição:** O sistema deve permitir responder ao lembrete de consulta confirmando, cancelando ou iniciando o fluxo de remarcação diretamente pela interface.

### RF-P06 — Processamento de pagamentos

- **Prioridade:** Essencial
- **Descrição:** O sistema deve processar pagamentos via Pix (Copia e Cola / QR Code) e Cartão de Crédito com cobrança recorrente.

### RF-P07 — Controle de pacotes mensais

- **Prioridade:** Desejável
- **Descrição:** O sistema deve exibir uma linha do tempo ou calendário que mostre o consumo das sessões do mês (sessões pagas, realizadas e futuras).

### RF-P08 — Termo de consentimento e contratos

- **Prioridade:** Essencial
- **Descrição:** O sistema deve apresentar e registrar a assinatura de um contrato digital referente ao tratamento de dados e uso da plataforma.

## Requisitos Não Funcionais

### RNF-P01 — Autenticação em Múltiplos Fatores (MFA)

- **Prioridade:** Essencial
- **Descrição:** O sistema deve exigir, além da senha, um código SMS ou biometria para autorizar o acesso aos dados na plataforma, garantindo a privacidade do paciente.

### RNF-P02 — Usabilidade (Apreensibilidade)

- **Prioridade:** Essencial
- **Descrição:** A interface inicial do paciente deve apresentar botões autoexplicativos, grandes e explícitos para suas funções primárias (ex: "Agendar", "Histórico").

### RNF-P03 — Acessibilidade Mobile

- **Prioridade:** Essencial
- **Descrição:** A plataforma deve funcionar perfeitamente em Smartphones (via App ou Navegador responsivo), sendo este o dispositivo preferencial de acesso.

## Regras de Domínio

### RD-P01 — Ciência de Tratamento de Dados (LGPD)

- **Prioridade:** Essencial
- **Descrição:** O paciente não poderá utilizar os recursos do portal sem antes assinar ou dar aceite no contrato digital que explica o uso dos seus dados sensíveis.

### RD-P02 — Confirmação de Segurança Pós-Pagamento

- **Prioridade:** Essencial
- **Descrição:** Após o pagamento, o paciente deve receber imediatamente uma confirmação e conseguir visualizar sua consulta na agenda do profissional.

## Restrições

### R-P01 — Anonimização na Agenda

- **Prioridade:** Essencial
- **Descrição:** Ao visualizar a agenda do profissional, o paciente só pode ver os seus próprios dados. Os demais horários ocupados devem aparecer de forma anônima.

### R-P02 — Adequação a Dispositivos Móveis

- **Prioridade:** Essencial
- **Descrição:** Telas complexas como recibos e históricos precisam ser adaptadas para exibição clara em telas menores de celular.

## Dados Principais

| ID     | Descrição                                                                                         |
| ------ | ------------------------------------------------------------------------------------------------- |
| DD-P01 | Dados de agendamento: Data, horário, profissional escolhido, status de presença.                  |
| DD-P02 | Dados financeiros: Pacotes mensais, emissão de recibos para IR, forma de pagamento (Pix, Cartão). |
| DD-P03 | Dados de segurança: Senha, biometria, código SMS de acesso.                                       |
| DD-P04 | Dados contratuais: Aceite do contrato digital, log de consentimento LGPD.                         |
