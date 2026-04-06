# Lista de Requisitos Geral (Consolidada)

## Requisitos Funcionais (RF)

| **ID**    | **Requisito**                                                                                                                           | **Prioridade** | **Fonte (Origem)**                       |
| --------- | --------------------------------------------------------------------------------------------------------------------------------------- | -------------- | ---------------------------------------- |
| **RF-01** | **Gestão de Prontuário Eletrónico:** Registo obrigatório com identificação, queixa, procedimentos técnicos, evolução e encaminhamentos. | Essencial      | Resolução CFP 001/2009; Base Legal       |
| **RF-02** | **Acesso e Exportação do Titular:** Garantir ao utilizador o acesso integral e a exportação das informações registadas no prontuário.   | Essencial      | LGPD; Resolução CFP 001/2009; Base Legal |
| **RF-03** | **Gestão de Consentimento:** Exigir aceite de Termo de Consentimento (TCLE) antes da coleta de dados sensíveis com registo de log.      | Essencial      | LGPD; Base Legal; Questionário Psicóloga |
| **RF-04** | **Agendamento Autónomo:** Permitir que o paciente aceda à agenda do profissional e marque consultas sem intermediação humana.           | Essencial      | Requisitos Paciente; Análise Amplimed    |
| **RF-05** | **Validação e-Psi:** Validar e exibir o registo do profissional no Cadastro e-Psi para autorização de atendimentos online.              | Essencial      | Resolução CFP 011/2018; Base Legal       |
| **RF-06** | **Ferramentas TCC Core:** Implementar cartões de enfrentamento digitais e registo de pensamentos e sentimentos (RDPD).                  | Desejável      | Requisitos Paciente (TCC)                |
| **RF-07** | **Escalas Clínicas:** Implementar ferramentas visuais (0 a 100%) como SUDS e Contínuo Cognitivo para quantificação emocional.           | Desejável      | Requisitos Paciente (TCC)                |
| **RF-08** | **Emissão de Documentos Fiscais:** Funcionalidade para geração de recibos para IRS e histórico organizado de consultas.                 | Essencial      | Requisitos Paciente                      |
| **RF-09** | **Lembretes Automatizados:** Envio de avisos de sessão via WhatsApp com 24h de antecedência e links de reunião.                         | Essencial      | Requisitos Paciente; Análise Amplimed    |
| **RF-10** | **Direitos do Titular (Retificação):** Interface para que o paciente solicite a correção de dados incompletos ou inexatos.              | Essencial      | LGPD; Base Legal                         |
| **RF-11** | **Tarefas de Casa:** Área para o psicólogo atribuir lições e o paciente respondê-las dentro da plataforma.                              | Desejável      | Requisitos Paciente (TCC)                |
| **RF-12** | **Apoio com IA:** Chatbot para suporte emocional e ensaios comportamentais sob supervisão do profissional.                              | Desejável      | Requisitos Paciente (TCC); Base Legal    |
| **RF-13** | **Psicoeducação e Liberação:** Repositório de conteúdos educativos com controlo de liberação gradual pelo terapeuta.                    | Desejável      | Requisitos Paciente (TCC)                |
| **RF-14** | **Download de Documentos:** Possibilitar o download de laudos, pareceres e outros documentos oficiais pelo paciente.                    | Essencial      | Requisitos Paciente (TCC)                |
| **RF-15** | **Interface Infantil:** Opção de alternar para uma interface lúdica e amigável para pacientes crianças.                                 | Desejável      | Requisitos Paciente (TCC)                |
| **RF-16** | **Cancelamento e Remarcação Ágil:** Permitir o cancelamento de consultas com visualização imediata de novos horários disponíveis.       | Essencial      | Requisitos Paciente; Análise Amplimed    |

## Requisitos Não Funcionais (RNF)

| **ID**     | **Requisito**                                                                                                                                                   | **Prioridade** | **Fonte(s)**                                |
| ---------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------- | ------------------------------------------- |
| **RNF-01** | **Criptografia de Ponta a Ponta:** Por tratar dados sensíveis de saúde, todas as comunicações e o banco de dados devem ser criptografados.                      | Essencial      | LGPD Art. 6 (VII); Questionário Psicóloga   |
| **RNF-02** | **Autenticação Multi-fator (MFA):** Suportar métodos ágeis e seguros como biometria ou PIN para acesso via dispositivos móveis.                                 | Importante     | ISO 27002 (8.5); Questionário Psicóloga     |
| **RNF-03** | **Disponibilidade e Backup:** Garantir rotinas automáticas de backup e alta disponibilidade para evitar perda de registos obrigatórios.                         | Essencial      | ISO 27002 (8.13); Questionário Psicóloga    |
| **RNF-04** | **Usabilidade (Foco em Cliques):** Interface intuitiva com o mínimo de cliques para o registo diário de prontuários, visando reduzir a resistência operacional. | Essencial      | Questionário Psicóloga                      |
| **RNF-05** | **Responsividade:** O sistema deve ser totalmente funcional em dispositivos móveis (tablets e smartphones).                                                     | Essencial      | Questionário Psicóloga; Requisitos Paciente |
| **RNF-06** | **Transcrição com IA:** Após uma sessão acontecer, o sistema deve transcrever automaticamente e gerar o prontuário da sessão.                                   | Desejável      | Entrevista Psicóloga                        |
| **RNF-07** | **Conformidade com o CFP:** A estrutura do prontuário eletrônico deve seguir rigorosamente as informações obrigatórias exigidas pela Resolução CFP n° 001/2009. | Essencial      | Resolução CFP 001/2009                      |

## Regras de Negócio e Restrições (RN e REST)

| **ID**      | **Descrição**                                                                                                                                                      | **Fonte(s)**                                          |
| ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------- |
| **RN-01**   | **Prazo de Guarda Documental:** Impedir a exclusão definitiva de prontuários por um período mínimo de **5 anos** após o último atendimento.                        | Res. CFP 001/2009; Base Legal; Questionário Psicóloga |
| **RN-02**   | **Sigilo na Comunicação Externa:** É proibido o tráfego de conteúdos de evolução clínica via WhatsApp; o canal deve limitar-se a dados administrativos.            | Base Legal; Código de Ética                           |
| **RN-03**   | **Anonimização de Agenda:** Na visualização de agendas compartilhadas, o paciente só pode ver os seus dados; horários de terceiros devem aparecer como "ocupados". | Base Legal; Requisitos Paciente                       |
| **RN-04**   | **Atendimento de Menores:** Exigir obrigatoriamente o registo e consentimento de pelo menos um responsável legal para pacientes menores de idade.                  | Código de Ética Art. 8; Base Legal                    |
| **REST-01** | **Custos Operacionais:** A infraestrutura em nuvem deve ser mantida ao mínimo para viabilizar um modelo de assinatura de baixo custo.                              | Questionário Psicóloga                                |
| **REST-02** | **Segregação em Equipes:** Em ambientes multiprofissionais, o prontuário deve ser único, mas informações sigilosas são restritas ao psicólogo responsável.         | Código de Ética Art. 12; Base Legal                   |
