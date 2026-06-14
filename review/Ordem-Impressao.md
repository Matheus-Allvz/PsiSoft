# Organização e Ordem de Impressão do Trabalho Final (PsiSoft)

Este documento mapeia exatamente como os arquivos avulsos do projeto devem ser compilados, ordenados e formatados em um **Documento Mestre Único** (ex: no Microsoft Word, Google Docs ou LaTeX) para que a impressão final atenda rigorosamente à solicitação estipulada para a entrega.

---

## Estrutura do Documento Impresso

**Capa, Folha de Rosto e Sumário** (Páginas pré-textuais padrão acadêmico/ABNT).

### 1. Especificação de Requisitos (IEEE830 ou EOR)
Nesta seção principal, o texto deve fluir incorporando os seguintes artefatos na exata ordem:

- [ ] **Lista de requisitos funcionais e não funcionais**
  *Arquivo Fonte:* `docs/requirements/specification/lista-requisitos-geral-consolidada.md`
- [ ] **Diagrama de casos de uso**
  *Arquivo Fonte:* Imagem extraída de `docs/requirements/specification/use-cases/diagrama-casos-de-uso.pdf`
- [ ] **Todos os casos de uso descritos no formato completo**
  *Arquivo Fonte:* `docs/architecture/c2-design-detalhado/02-descricoes-casos-uso.md` (Após ser preenchido)
- [ ] **Protótipo de todas as telas com explicação e conexão com caso de uso**
  *Arquivo Fonte:* `docs/architecture/c2-design-detalhado/03-prototipos-interface.md` + Imagens da pasta `ui-prototypes`
- [ ] **Modelo Entidade-Relacionamento (MER)**
  *Arquivo Fonte:* `docs/architecture/c3-componentes-e-dados/assets/modelo-logico.png` (Inserir a imagem com legenda)
- [ ] **Matriz de Rastreabilidade Horizontal**
  *Arquivo Fonte:* `docs/requirements/specification/matriz-rastreabilidade-horizontal.md` (Após ser construída)
- [ ] **Anexos (Requisitos)**
  - *Descrição completa das entrevistas:* Mesclar os arquivos `.md` e `.pdf` de `docs/requirements/elicitation/interviews/`.
  - *Perguntas e respostas dos questionários:* Mesclar conteúdos de `docs/requirements/elicitation/questionnaires/`.
  - *Síntese e análise do sistema análogo:* `docs/requirements/elicitation/analogous-systems/sintese-amplimed.md`.
  - *Dicionário de Dados MER:* Transcrever tabelas do `docs/architecture/c3-componentes-e-dados/assets/dicionario-de-dados.docx`.

---

### 2. Especificação de Arquitetura (Simplificação 42010)

- [ ] **1 Introdução**
  *Arquivo Fonte:* `docs/architecture/00-introducao-arquitetura.md` (A criar)
- [ ] **2 Definição e apresentação dos stakeholders e respectivos interesses**
  *Arquivo Fonte:* `docs/architecture/00-introducao-arquitetura.md` (A criar)
- [ ] **3 Principais requisitos não funcionais atendidos, como estão sendo tratados**
  *Arquivo Fonte:* `docs/architecture/00-introducao-arquitetura.md` (A criar)
  
- [ ] **4 Nível C1**
  - *Explicação geral e Diagrama de implantação:* `docs/architecture/c1-alto-nivel/01-implantacao.md` (Certificando de exportar o código PlantUML gerando a imagem `.png` para o Word).
  
- [ ] **5 Nível C2**
  - *Explicação geral e Diagrama de Pacotes com racional:* `docs/architecture/c2-design-detalhado/01-diagrama-pacotes.md` (Após ser preenchido) e sua respectiva imagem gráfica.
  
- [ ] **6 Nível C3**
  - *Explicação geral e Diagrama de componentes para cada pacote:* `docs/architecture/software/Diagrama de Componentes por Pacote/Componentes.md` e colar os gráficos de cada `.puml`.
  - *Diagrama UML de Sequência (DSeq) com mensagens entre componentes:* `docs/architecture/c3-componentes-e-dados/01-sequencia-componentes.md` + Imagem `.svg/.png`.
  
- [ ] **7 Nível C4**
  - *Explicação geral do Nível C4:* Introdução de `docs/architecture/c4-implementacao/01-contratos-operacao.md`
  - *Esquema Operacional de BD:* `docs/architecture/c4-implementacao/02-esquema-banco.md` (Tabelas compiladas)
  - *Para cada componente - Diagrama de classes:* `docs/architecture/c4-implementacao/03-diagramas-classes.md` (Após criados e exportados para PNG)
  - *Para cada operação em uma Classe - Diagrama de sequência (2 exemplos):* `docs/architecture/c4-implementacao/04-sequencia-classes.md`
  - *Contrato de operação (Capítulo 11 - Larman - 1 exemplo):* Extraído de `docs/architecture/c4-implementacao/01-contratos-operacao.md`.

---

### 3. Anexos Finais

- [ ] **8 Anexo - Matriz de Rastreabilidade Vertical**
  *Arquivo Fonte:* `docs/requirements/specification/matriz-rastreabilidade-vertical.md` (Após ser construída ligando Req -> Componente -> Código Rust).

---

## O que falta para a impressão? (Passos Finais)

Além de executar as tarefas de "Preenchimento/Criação" levantadas no plano anterior, para ter o trabalho físico nas mãos você precisará:

1. **Exportação de Diagramas:** Todo diagrama escrito em código (`.puml`) precisará ser renderizado para imagem (`.png` ou `.jpeg`) para ser colado em um documento texto.
2. **Mesclagem de Formatos (Agrupamento):** Atualmente o projeto possui arquivos `.md`, `.docx`, e `.pdf`. Será necessário um trabalho de formatação manual: usar um processador como Microsoft Word ou LaTeX para "copiar e colar" o conteúdo de cada Markdown e PDF na ordem listada acima.
3. **Paginação e Estilização:** Ajustar quebras de página (diagramas grandes podem quebrar no meio do texto), garantir sumário automatizado e margens acadêmicas legíveis.
4. **Revisão Final em PDF:** Salvar este grande documento agrupado como um único PDF mestre e realizar uma verificação visual antes da impressão.
