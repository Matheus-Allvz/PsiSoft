## 3.2 Diagramas de Componentes por Pacote

Além do diagrama geral de componentes da plataforma PsiSoft, foram elaborados diagramas de componentes específicos para cada pacote do sistema. Essa abordagem permite detalhar as responsabilidades internas de cada módulo, evidenciando seus componentes, interfaces fornecidas e dependências, facilitando a compreensão, manutenção e evolução da arquitetura.

Os pacotes modelados foram:

* **Agendamento** (`dcmp-agendamento.puml`)
* **Consulta** (`dcmp-consulta.puml`)
* **Contrato** (`dcmp-contrato.puml`)
* **Gestão Financeira** (`dcmp-gestao-financeira.puml`)
* **Notificação** (`dcmp-notificacao.puml`)
* **Prontuário** (`dcmp-prontuario.puml`)
* **Sessão** (`dcmp-sessao.puml`)
* **Usuários** (`dcmp-usuarios.puml`)

A decomposição por pacotes segue o princípio da separação de responsabilidades (*Separation of Concerns*), permitindo analisar individualmente cada subsistema do domínio de negócio e suas integrações com os demais componentes da aplicação. Dessa forma, os diagramas complementam a visão arquitetural de alto nível apresentada anteriormente, fornecendo uma visão mais detalhada da estrutura interna do sistema.
