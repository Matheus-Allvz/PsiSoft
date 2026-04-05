# Especificação de Requisitos - Conformidade Legal e Normativa (PsiSoft)

## 1. Requisitos Funcionais (RF)

### Gestão de Prontuário e Documentação
* **RF-L01 — Registro Documental Obrigatório:** O sistema deve fornecer campos obrigatórios para o registro da prestação de serviços, incluindo: identificação do paciente, diagnóstico ou queixa, procedimentos técnicos adotados, evolução do caso e registro de encaminhamentos.
* **RF-L02 — Acesso Integral do Paciente:** O portal deve disponibilizar uma funcionalidade que permita ao paciente ou seu representante legal visualizar e exportar a íntegra das informações registradas em seu prontuário a qualquer momento.
* **RF-L03 — Gestão de Consentimento (LGPD):** O sistema deve exigir a aceitação de um Termo de Consentimento Livre e Esclarecido (TCLE) antes do início da coleta de qualquer dado sensível, registrando o log de aceite com data e hora.
* **RF-L04 — Cadastro e-Psi:** Para atendimentos online, o sistema deve permitir a validação e exibição do registro do profissional no Cadastro e-Psi para garantir a autorização da prestação de serviços por meios tecnológicos.
* **RF-L05 — Direitos do Titular:** Deve existir uma interface para que o paciente solicite a correção de dados incompletos, inexatos ou desatualizados, conforme previsto na legislação de proteção de dados.

### Apoio e Intervenção
* **RF-L06 — Supervisão de IA:** Caso o sistema utilize inteligência artificial para interação, esta deve ser restrita a suportes administrativos ou psicoeducação pré-definida, garantindo que intervenções clínicas e ensaios terapêuticos sejam realizados exclusivamente pelo profissional.

## 2. Requisitos Não Funcionais (RNF)

### Segurança e Confidencialidade
* **RNF-L01 — Criptografia de Dados Sensíveis:** Todos os dados de saúde e registros de sessões devem ser criptografados tanto em repouso quanto em trânsito, utilizando protocolos de segurança avançados para mitigar riscos de vazamento.
* **RNF-L02 — Trilha de Auditoria Inviolável:** O sistema deve manter registros automáticos (logs) de todos os acessos, alterações ou exclusões nos prontuários, identificando o autor e o momento exato da ação para fins de prova idônea.
* **RNF-L03 — Autenticação de Múltiplos Fatores (MFA):** O acesso à área do profissional e a documentos sensíveis deve exigir uma segunda camada de autenticação (como código via SMS ou aplicativo) para reforçar o controle de acesso.
* **RNF-L04 — Disponibilidade e Backup:** A arquitetura do sistema deve garantir rotinas automáticas de backup e alta disponibilidade para evitar a perda de registros clínicos obrigatórios.

## 3. Regras de Negócio (RN)

* **RN-L01 — Prazo de Guarda Documental:** O sistema deve impedir a exclusão definitiva de registros e prontuários por um período mínimo de 5 (cinco) anos após o último atendimento, garantindo a preservação histórica exigida pelas normas profissionais.
* **RN-L02 — Prontuário Único em Equipes:** Em ambientes multiprofissionais, o registro deve ser realizado em prontuário único, porém o sistema deve restringir a visualização de informações estritamente sigilosas apenas ao psicólogo responsável.
* **RN-L03 — Atendimento de Menores:** Para o cadastro de pacientes menores de idade, o sistema deve exigir obrigatoriamente o registro e o consentimento eletrônico de ao menos um responsável legal.

## 4. Restrições (R)

* **R-L01 — Sigilo na Comunicação Externa:** O envio de lembretes ou comunicações via aplicativos de mensagens (WhatsApp) deve ser limitado a dados administrativos (data e hora), sendo proibido o tráfego de conteúdos de evolução clínica por esses canais.
* **R-L02 — Anonimização de Agenda:** Na visualização de agendas compartilhadas ou públicas, o sistema deve anonimizar completamente os dados de outros pacientes, exibindo apenas o status de horário ocupado.
* **R-L03 — Localização da Infraestrutura:** O tratamento de dados deve ocorrer em ambientes que garantam o cumprimento das diretrizes de privacidade e proteção de dados nacionais.