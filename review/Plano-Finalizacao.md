# Plano de Finalização do Projeto (PsiSoft)

Os planos descritos anteriormente nas revisões cobriam as falhas isoladas, mas não estavam ordenados em um pipeline sequencial e exaustivo até a entrega final. Este documento unifica **todas as etapas necessárias**, do ponto atual até a formatação da entrega da Parte III, garantindo que o projeto atenda perfeitamente aos requisitos estipulados no arquivo `task-FINAL.md` e ao modelo de referência 42010.

Abaixo, as tarefas estão divididas em fases de execução com o mapeamento de dependências e responsáveis.

---

## FASE 1: Especificação de Requisitos (IEEE830)

### Etapa 1.1: Detalhamento Completo dos Casos de Uso
- **Arquivo Relacionado:** `docs/architecture/c2-design-detalhado/02-descricoes-casos-uso.md`
- **Interdependências:** Nenhuma.
- **Responsável:** [MATHEUS]
- **Descrição da Tarefa:** O arquivo atual encontra-se vazio. É necessário descrever **todos** os casos de uso mapeados no diagrama UML em formato completo e expandido, seguindo o padrão Larman (Capítulo 6).



---

## FASE 2: Especificação de Arquitetura - Nível C1 (Contexto)

### Etapa 2.1: Documentação Introdutória e Stakeholders
- **Arquivo Relacionado:** `docs/architecture/00-introducao-arquitetura.md` (Novo)
- **Interdependências:** Nenhuma.
- **Responsável:** [LAVINIA]
- **Descrição da Tarefa:** Redigir as Seções 1, 2 e 3 exigidas pelo `task-FINAL.md`: Introdução/Propósito, Definição dos Stakeholders com seus interesses, e a explicação de como os principais Requisitos Não Funcionais (segurança, performance, etc.) estão sendo resolvidos na arquitetura.

### Etapa 2.2: Ajuste do Diagrama de Implantação
- **Arquivo Relacionado:** `docs/architecture/c1-alto-nivel/01-implantacao.md`
- **Interdependências:** Nenhuma.
- **Responsável:** [MATHEUS]
- **Descrição da Tarefa:** Substituir no texto e no código PlantUML a referência à stack genérica "Node.js / JVM" pela linguagem real adotada no backend: **Rust (Axum, Tokio, Sqlx)**, validando os nós de implantação.

### Etapa 2.3: Diagramas de Atividade (BPMN)
- **Arquivo Relacionado:** `docs/architecture/c1-alto-nivel/02-diagramas-atividade.md` (Novo)
- **Interdependências:** Etapa 1.1 (Entendimento dos fluxos dos Casos de Uso).
- **Responsável:** [MATHEUS E MARCOS]
- **Descrição da Tarefa:** Criar os diagramas de atividade na notação BPMN para cada processo de negócio central do sistema (ex: Agendamento, Consulta, Faturamento).

---

## FASE 3: Especificação de Arquitetura - Nível C2 e C3

### ✅ Etapa 3.1: Preenchimento do Racional do Diagrama de Pacotes (C2) - CONCLUÍDA
- **Arquivo Relacionado:** `docs/architecture/c2-design-detalhado/01-diagrama-pacotes.md` e PDF.
- **Interdependências:** Nenhuma.
- **Responsável:** [DAVI]
- **Descrição da Tarefa:** O arquivo disposto como PDF em Assets atende a este requisito arquitetural. (Status: Concluído).

### Etapa 3.2: Refinamento dos Diagramas de Componentes (C3)
- **Arquivos Relacionados:** `docs/architecture/software/Diagrama de Componentes por Pacote/Componentes.md` e arquivos `*.puml`
- **Interdependências:** Etapa 3.1.
- **Responsável:** [MATHEUS :(, VISTO QUE REFLETE O BACKEND]
- **Descrição da Tarefa:** Atualizar os componentes visuais para refletirem os módulos concretos implementados (ex: `api::auth`, `domain::user`) e modelar a comunicação em blocos (Ports & Adapters / Layered), saindo da abstração genérica para a realidade da Stack.

---

## FASE 4: Especificação de Arquitetura - Nível C4 (Código e BD)

### Etapa 4.1: Documentação do Esquema Operacional de BD
- **Arquivo Relacionado:** `docs/architecture/c4-implementacao/02-esquema-banco.md` (Novo)
- **Interdependências:** Requer leitura do `script-psisoft.sql`.
- **Responsável:** [NICOLE]
- **Descrição da Tarefa:** Transcrever o DDL (SQL) existente em uma tabela textual explicativa listando Nome da Tabela, Atributos, Tipos Reais (PostgreSQL/SQL) e Tamanhos, com uma breve descrição do propósito de cada uma.

### Etapa 4.2: Construção dos Diagramas de Classes
- **Arquivo Relacionado:** `docs/architecture/c4-implementacao/03-diagramas-classes.md` (Novo)
- **Interdependências:** Etapa 3.2.
- **Responsável:** [MATHEUS]
- **Descrição da Tarefa:** Criar, para cada componente C3, o diagrama UML detalhando as Entidades do Domínio (Structs em Rust), seus métodos (Traits/Impls) e seus atributos com tipagem nativa.

### Etapa 4.3: Diagramas de Sequência por Operação
- **Arquivo Relacionado:** `docs/architecture/c4-implementacao/04-sequencia-classes.md` (Novo)
- **Interdependências:** Etapa 4.2.
- **Responsável:** [MATHEUS]
- **Descrição da Tarefa:** Selecionar **2 exemplos** de operações essenciais (ex: Cadastrar Paciente e Agendar Consulta) e criar um Diagrama de Sequência de baixo nível focado na interação das "classes"/arquivos do código (Controller -> Service -> Model -> Database).

### Etapa 4.4: Contratos de Operação (Larman Cap. 11)
- **Arquivo Relacionado:** `docs/architecture/c4-implementacao/01-contratos-operacao.md`
- **Interdependências:** Etapa 4.2.
- **Responsável:** [MATHEUS]
- **Descrição da Tarefa:** Revisar os contratos existentes para garantir que a tipagem mencionada faça sentido com o código Rust (`VARCHAR` vira `String`, `INT` vira `i32`/`i64`). Garantir que haja pelo menos o número mínimo exigido validado.

---

## FASE 5: Entrega Final e Rastreabilidade

### Etapa 5.1: Matriz de Rastreabilidade Vertical (Anexo 8)
- **Arquivo Relacionado:** `docs/requirements/specification/matriz-rastreabilidade-vertical.md` (Novo)
- **Interdependências:** Conclusão das Fases 1 a 4.
- **Responsável:** [ ]
- **Descrição da Tarefa:** Elaborar a Matriz exigida no Anexo 8 que demonstre a linha condutora desde o **Requisito Original -> Componente Arquitetural C3 -> Módulo/Arquivo de Código Fonte (Rust)**.

### Etapa 5.2: (Opcional/Se Exigido) Implementação do Código Espelhado
- **Arquivos Relacionados:** `backend/src/*` e `frontend/*`
- **Interdependências:** Conclusão dos Contratos (Etapa 4.4).
- **Responsável:** [MATHEUS]
- **Descrição da Tarefa:** Como o backend atual foca apenas em `auth/user` e a arquitetura foca em `agendamento`, deve-se escrever a base dos end-points de Agendamento, Consulta e Pagamento em Rust para validar que o Código (Nível C4) e a Documentação são coerentes na entrega. Além disso, criar a base do Frontend além de `agendamentos.html`.

---

## FASE 6: Formatação e Impressão para Entrega

Esta fase garante que todos os arquivos soltos do projeto sejam compilados em um único Documento Mestre formatado nos padrões exigidos para a entrega física (impressão).

### Etapa 6.1: Exportação Gráfica dos Diagramas
- **Arquivo Relacionado:** Todos os `.puml` do Nível C1, C2 e C3.
- **Interdependências:** Conclusão das Fases 2, 3 e 4.
- **Responsável:** [ ]
- **Descrição da Tarefa:** Renderizar os códigos de diagramas PlantUML em arquivos de imagem definitivos (`.png` ou `.jpg`) garantindo alta resolução para que o texto interno seja legível no papel.

### Etapa 6.2: Mesclagem no Documento Mestre
- **Arquivo Relacionado:** Arquivo Mestre (ex: Word, Google Docs ou LaTeX)
- **Interdependências:** Etapa 6.1 e todas as Fases textuais.
- **Responsável:** [ ]
- **Descrição da Tarefa:** Extrair o texto de todos os arquivos `.md` criados nas fases anteriores e colar no Documento Mestre, rigorosamente na ordem estrutural definida no arquivo de revisão `Ordem-Impressao.md`. Incluir manualmente os textos extraídos dos Anexos PDF (como entrevistas) ou prepará-los para agrupamento.

### Etapa 6.3: Formatação Acadêmica (Capa a Anexos)
- **Arquivo Relacionado:** Arquivo Mestre
- **Interdependências:** Etapa 6.2.
- **Responsável:** [ ]
- **Descrição da Tarefa:** Criar Capa, Folha de Rosto e gerar o Sumário automatizado. Ajustar fontes, margens, estilos de títulos (H1, H2, H3) e, principalmente, revisar e consertar quebras de página para que diagramas não fiquem cortados ao meio na hora da impressão.

### Etapa 6.4: Geração do PDF Final e Impressão
- **Arquivo Relacionado:** `Entrega-Final-PsiSoft.pdf`
- **Interdependências:** Etapa 6.3.
- **Responsável:** [ ]
- **Descrição da Tarefa:** Exportar o Documento Mestre formatado para PDF, fazer uma última leitura em tela inteira, e então enviá-lo para a gráfica/impressora física. Entregar.

---
*Este documento unificado garante controle gerencial e previsibilidade técnica até o momento de exportação e submissão da arquitetura final.*
