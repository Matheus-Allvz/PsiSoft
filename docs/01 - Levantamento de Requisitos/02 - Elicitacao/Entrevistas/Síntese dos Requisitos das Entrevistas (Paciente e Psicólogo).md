# Síntese dos Requisitos das Entrevistas (Paciente e Psicólogo)

Este documento detalha os requisitos funcionais e não funcionais levantados a partir das entrevistas com usuários, focando em uma abordagem de Terapia Cognitivo-Comportamental (TCC) e facilitação administrativa.

---

## 1. Requisitos Funcionais (RF)

_Descrevem as funcionalidades e serviços que o sistema deve fornecer._

### Gestão Terapêutica (Core do Aplicativo)

- **RF01 - Cartões de Enfrentamento:** O sistema deve permitir que o psicólogo crie cartões digitais com técnicas de respiração, lições ou insights durante a sessão para que apareçam na tela inicial do paciente.
- **RF02 - Registro de Pensamentos e Sentimentos:** O paciente deve ser capaz de registrar o que está pensando e sentindo para que o terapeuta possa visualizar e oferecer feedback.
- **RF03 - Tarefas de Casa:** O sistema deve possuir uma área específica para o psicólogo atribuir lições e o paciente respondê-las dentro da plataforma.
- **RF04 - Escalas Clínicas (SUDS e Contínuo Cognitivo):** Implementar ferramentas visuais (escala de 0 a 100%) para o paciente quantificar níveis de ansiedade e flexibilizar pensamentos rígidos/polarizados.
- **RF05 - Chatbot de Apoio com IA:** Uma inteligência artificial que emule o modo de fala do psicólogo para responder dúvidas sobre atividades, oferecer apoio emocional e realizar ensaios comportamentais (simulação de situações difíceis).
- **RF06 - Liberação Gradual de Conteúdo:** Funcionalidades e técnicas de TCC só devem ficar visíveis para o paciente após a liberação manual por parte do psicólogo.

### Financeiro e Administrativo

- **RF07 - Gerador de Código PIX:** O sistema deve gerar códigos de pagamento "Copia e Cola" para as sessões, facilitando a quitação financeira.
- **RF08 - Lembretes Automáticos:** Disparar notificações de lembrete para horários de consultas e lembretes de pagamento com antecedência de 24 horas.
- **RF09 - Download de Documentos:** Área segura para o paciente baixar laudos e outros documentos oficiais emitidos pelo profissional.

### Personalização

- **RF10 - Interface Infantil:** Opção de alternar para uma interface lúdica, com jogos e visual amigável, caso o psicólogo sinalize que o paciente é uma criança.

---

## 2. Requisitos Não Funcionais (RNF)

_Descrevem as restrições e qualidades do sistema._

- **RNF01 - Segurança e Privacidade de Dados:** Garantir que os dados registrados sejam estritamente privados e visíveis apenas pelo binômio psicólogo-paciente.
- **RNF02 - Autenticação de Camada Dupla:** O sistema deve exigir login e senha para acesso geral, e uma **senha secundária diferenciada** para acessar ou baixar documentos sensíveis (como laudos).
- **RNF03 - Usabilidade e Simplicidade:** A interface deve ser intuitiva e "limpa", apresentando apenas as funcionalidades essenciais para não sobrecarregar o usuário.
- **RNF04 - Interoperabilidade com WhatsApp:** O sistema deve coexistir com o WhatsApp para comunicações diretas e envio de links de reunião (Google Meet), respeitando a preferência do usuário por um contato mais "humano".
- **RNF05 - Controle de Acesso Restrito:** O sistema não deve permitir o uso por pessoas que não sejam pacientes ativos, evitando que as ferramentas terapêuticas sejam utilizadas sem supervisão profissional.
- **RNF06 - Disponibilidade de Lembretes:** Os lembretes automáticos devem ser enviados preferencialmente via WhatsApp ou integração que garanta a visualização imediata.

- **RNF07 - Transcrição com IA:** Após uma sessão acontecer, o sistema deve transcrever automaticamente e gerar o prontuário da sessão.

- **RNF08 - Conformidade com o CFP:** A estrutura do prontuário eletrônico deve seguir rigorosamente as informações obrigatórias exigidas pela Resolução CFP n° 001/2009.
