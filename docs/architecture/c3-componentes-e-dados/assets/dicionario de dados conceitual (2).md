# Dicionário de Dados Conceitual (Glossário de Negócios)

| Entidade / Tabela | Atributo | Descrição / Significado no Negócio |
| :--- | :--- | :--- |
| **Usuario** | id | Código único de identificação do colaborador na clínica. |
| | nome | Nome completo do colaborador. |
| | login | Nome de usuário utilizado para entrar no sistema. |
| | senha_hash | Senha de acesso ao sistema (armazenada de forma segura). |
| | perfil | Função do colaborador no sistema (Psicólogo, Secretária ou Contador). |
| | email | E-mail para contato e recuperação de acesso. |
| | telefone | Número de telefone para contato. |
| | status | Situação do colaborador na clínica (Ativo ou Inativo). |
| **Psicologo** | crp | Número do registro profissional no Conselho (CRP). |
| | validador_epsi | Chave de validação do cadastro no e-Psi para atendimentos online. |
| | fk_usuario_id | Vínculo com os dados básicos e de acesso do colaborador. |
| **Secretaria** | fk_usuario_id | Vínculo com os dados básicos e de acesso do colaborador. |
| **Contador** | fk_usuario_id | Vínculo com os dados básicos e de acesso do colaborador. |
| **Paciente** | id | Código único de identificação do paciente. |
| | nome | Nome completo do paciente. |
| | cpf | Cadastro de Pessoa Física (CPF) do paciente. |
| | data_nasc | Data de nascimento (usada para calcular a idade e adequar o perfil do sistema). |
| | genero | Identidade de gênero do paciente. |
| | endereco | Endereço residencial completo. |
| | email | E-mail para contato e acesso ao portal exclusivo do paciente. |
| | telefone | Número principal para contato telefônico. |
| | senha_hash | Senha pessoal para acesso ao portal do paciente. |
| | pin_biometria | Chave de validação para acesso rápido pelo aplicativo de celular (biometria/FaceID). |
| | status_menor | Indicador automático se o paciente é menor de idade. |
| | contato_wpp | Permissão para envio de mensagens e notificações via WhatsApp. |
| | fk_responsavel_id | Vínculo com o adulto responsável legal (obrigatório caso seja menor de idade). |
| | fk_prontuario_id | Vínculo direto com o histórico clínico (prontuário) do paciente. |
| **Responsavel** | id | Código de identificação do responsável legal. |
| | nome | Nome completo do adulto responsável. |
| | cpf | Cadastro de Pessoa Física (CPF) do responsável. |
| | parentesco | Grau de parentesco em relação ao paciente (ex: Pai, Mãe, Tutor). |
| | contato_wpp | Número de WhatsApp para envio de notificações, cobranças e contato de emergência. |
| **Contrato_LGPD** | id | Número de registro do consentimento assinado. |
| | data_aceite | Data e horário exatos em que o paciente concordou com os termos. |
| | ip_registro | Informação técnica do dispositivo usado para garantir a validade jurídica da assinatura. |
| | termo_tcle | Texto completo do contrato ou termo de consentimento que foi lido e aceito. |
| | fk_paciente_id | Paciente titular que assinou o documento. |
| **Prontuario** | id | Número de registro do prontuário clínico. |
| | data_abertura | Data do primeiro atendimento ou criação do documento. |
| | status | Situação atual do prontuário (Ativo para pacientes em terapia, ou Arquivado). |
| **Reg_Sessao** | id | Código de identificação do relato da sessão. |
| | queixa | Descrição do problema ou motivo relatado pelo paciente durante a consulta. |
| | obs_clinicas | Anotações e observações técnicas feitas pelo terapeuta. |
| | evolucao | Registro do progresso e desenvolvimento clínico do paciente na sessão. |
| | identificacao_sessao | Rótulo para organizar as sessões (ex: 'Sessão 01', 'Avaliação Inicial'). |
| | fk_prontuario_id | Prontuário onde estas anotações serão guardadas. |
| | fk_transcricao_id | Vínculo com o áudio e a transcrição gerada pela IA naquele dia (se houver). |
| **Transcricao** | id | Identificador do arquivo de transcrição. |
| | audio_path | Local seguro onde a gravação de voz da sessão está armazenada. |
| | texto_ia | Texto redigido automaticamente pela Inteligência Artificial a partir da fala. |
| **Consulta** | id | Número de identificação do agendamento. |
| | data_hora | Dia e horário marcados para o atendimento. |
| | presenca | Situação da consulta (Agendada, Realizada, Falta ou Cancelada). |
| | modalidade | Formato de atendimento escolhido (Presencial no consultório ou Online). |
| | fk_paciente_id | Paciente que será atendido. |
| | fk_psicologo_id | Profissional responsável por conduzir a consulta. |
| | fk_pacote_sessao_id | Pacote contratado do qual esta sessão será descontada (se aplicável). |
| | fk_pagamento_id | Vínculo com os dados de cobrança e recebimento referentes a esta consulta. |
| **Pagamento** | id | Código do registro financeiro da consulta. |
| | valor | Valor em Reais cobrado pelo atendimento. |
| | metodo | Forma de pagamento utilizada pelo paciente (PIX, Cartão de Crédito, Convênio). |
| | quitado | Indicador financeiro se o valor já foi recebido pela clínica. |
| | fk_doc_fiscal_id | Vínculo com a nota fiscal ou recibo emitido para este pagamento. |
| **Doc_Fiscal** | id | Número de controle do documento fiscal no sistema. |
| | tipo | Classificação do documento gerado (Recibo simples ou Nota Fiscal Eletrônica). |
| | data_emissao | Data em que o documento foi gerado e enviado ao paciente. |
| | arq_pdf | Arquivo PDF final do documento gerado. |
| **Atv_Terap** | id | Identificador da tarefa de casa terapêutica. |
| | categoria | Classificação do exercício (ex: Cartão de Enfrentamento, Questionário de humor). |
| | enunciado | Instruções detalhadas do que o paciente precisa fazer. |
| | resposta_pac | Resposta ou reflexão escrita pelo paciente através do portal. |
| | feedback_psi | Comentários e retornos do psicólogo sobre o exercício respondido. |
| | fk_psicologo_id | Psicólogo que recomendou a atividade. |
| | fk_paciente_id | Paciente que deve realizar a tarefa. |
| **Conteudo** | id | Identificador do material psicoeducativo no repositório. |
| | titulo | Nome do vídeo, artigo, jogo ou PDF. |
| | arq_midia | Arquivo ou link direto para acessar o material de apoio. |
| | perfil_interface | Classificação do público-alvo para filtrar materiais adequados (ex: Infantil, Adulto, Casal). |
| | fk_psicologo_id | Profissional que produziu ou adicionou o material no sistema. |
| **Acessa** | fk_paciente_id | Paciente que recebeu a liberação para ver o material. |
| | fk_conteudo_id | O material específico que foi disponibilizado. |
| | status_liberacao | Controle que permite ao psicólogo liberar os materiais gradualmente, conforme o andamento da terapia. |
| **Pacote_Sessao** | id | Código de identificação do pacote contratado. |
| | valor_total | Valor integral a ser pago pelo conjunto de sessões. |
| | total_sessoes | Quantidade total de sessões que o paciente comprou neste contrato. |
| | sessoes_utilizadas | Contador de quantas sessões do pacote já foram realizadas. |
| | data_validade | Prazo limite estipulado para que o paciente utilize todas as sessões compradas. |
| | fk_paciente_id | Paciente titular do pacote de sessões. |
| **Notificacao** | id | Identificador do aviso gerado pelo sistema. |
| | conteudo_msg | Texto da mensagem de lembrete ou cobrança. |
| | data_hora_disparo | Momento exato programado para o sistema enviar a mensagem. |
| | acao_resposta | Registro da interação do paciente com a mensagem (ex: Clicou em Confirmar ou Cancelar). |
| | fk_consulta_id | Consulta ou agendamento ao qual este lembrete pertence. |