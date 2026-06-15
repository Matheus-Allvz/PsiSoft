## 2. Diagramas de Atividade

Abaixo estão os diagramas de atividade (UML) para os principais fluxos de negócio do sistema PsiSoft, modelados conforme a implementação atual.

### 2.1 Cadastro de Paciente

```plantuml
@startuml
start
:Receber dados de Cadastro de Paciente;
:Iniciar Transação no Banco de Dados;
:Gerar hash da senha (Argon2);
:Definir login (CPF ou Email);
:Inserir Usuario (perfil='paciente');
:Inserir Paciente vinculado ao Usuario e Psicologo;
if (Possui Responsável Legal?) then (Sim)
  :Inserir Responsável Legal;
else (Não)
endif
:Inserir ConfiguracaoPaciente (Canal e Modalidade);
:Inserir ConsentimentoPaciente (TCLE, etc);
:Commit da Transação;
fork
  :Retornar ID do Paciente (HTTP 201);
fork again
  :Disparar Task Assíncrona;
  :Enviar E-mail de Boas-Vindas (Resend) com login e senha provisória;
end fork
stop
@enduml
```

### 2.2 Agendamento de Consulta

```plantuml
@startuml
start
:Receber Requisição de Agendamento;
if (Modalidade == Online?) then (Sim)
  :Buscar Validador e-Psi do Psicólogo;
  if (Validado no e-Psi?) then (Não)
    :Retornar Erro (EpsiNaoValidado);
    stop
  else (Sim)
  endif
else (Não, Presencial)
endif
:Verificar Conflito de Horário para o Psicólogo;
if (Horário Conflitante?) then (Sim)
  :Retornar Erro (ConflitoHorario);
  stop
else (Não)
endif
:Inserir Consulta (Status = Agendada);
:Retornar Dados da Consulta (HTTP 201);
stop
@enduml
```
