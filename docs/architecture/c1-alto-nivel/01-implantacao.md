## 1. Arquitetura de Alto Nível (Nível C1)

### 1.1 Diagrama de Implantação

A figura abaixo ilustra o Diagrama de Implantação do sistema PsiSoft, destacando a infraestrutura de hospedagem em nuvem e as fronteiras de comunicação com sistemas externos.

**Justificativa Arquitetural:**
Para garantir a conformidade com a Lei Geral de Proteção de Dados (LGPD) e as normativas do Conselho Federal de Psicologia (CFP), a arquitetura foi desenhada separando o servidor de aplicação do servidor de banco de dados, sendo este último isolado em uma rede privada (VPC). Toda a comunicação entre o dispositivo do cliente e a API, bem como as integrações essenciais com serviços de terceiros (WhatsApp, Cadastro e-Psi, Motor de IA e Gateway de Pagamento), ocorre exclusivamente sob o protocolo de segurança HTTPS/TLS 1.3. Os dados sensíveis no SGBD são protegidos por criptografia AES-256.

---

### Apêndice: Código-Fonte do Diagrama (PlantUML)

*Nota técnica: Este bloco de código serve para manutenção futura da arquitetura pela equipe e pode ser omitido na compilação final do LaTeX.*

```plantuml
@startuml
' Configurações visuais
skinparam node {
    BackgroundColor LightBlue
    BorderColor DarkBlue
}
skinparam artifact {
    BackgroundColor #E3F2FD
    BorderColor #1E88E5
}

' 1. Lado do Cliente
node "Dispositivo do Usuário\n(Smartphone / PC)" as Cliente <<device>> {
    node "Navegador / SO Mobile" <<executionEnvironment>> {
        artifact "Frontend PsiSoft\n(PWA/App)" as UI <<artefato>>
    }
}

' 2. Infraestrutura Principal (A Nuvem)
node "Ambiente Cloud PsiSoft\n(AWS / GCP - VPC)" as Nuvem <<device>> {

    node "Instância do Servidor de Aplicação" as Backend <<device>> {
        node "Runtime (ex: Node.js / JVM)" <<executionEnvironment>> {
            artifact "API PsiSoft Executável" as API <<artefato>>
        }
    }

    node "Instância de Banco de Dados" as DBServer <<device>> {
        node "SGBD (MySQL / PostgreSQL)" <<executionEnvironment>> {
            artifact "Esquema e Dados\nCriptografados (AES-256)" as Dados <<artefato>>
        }
    }
}

' 3. Serviços de Terceiros
node "Infraestrutura de Terceiros" as Terceiros <<device>> {

    node "Servidores CFP" <<device>> {
        artifact "API e-Psi" as ePsi <<artefato>>
    }

    node "Servidores Meta" <<device>> {
        artifact "API WhatsApp" as Wpp <<artefato>>
    }

    node "Provedor de IA" <<device>> {
        artifact "Motor de Transcrição" as IA <<artefato>>
    }

    node "Gateway de Pagamento" <<device>> {
        artifact "Processador Financeiro" as Pgto <<artefato>>
    }
}

' ==========================================
' Caminhos de Comunicação (Associações Físicas)
' ==========================================

' Comunicação Interna e Cliente
UI -- API : <<HTTPS / TLS 1.3>>
API -- Dados : <<TCP/IP / Rede Privada>>

' Integrações Externas
API -- ePsi : <<HTTPS / REST>>
API -- Wpp : <<HTTPS / REST>>
API -- IA : <<HTTPS / REST>>
API -- Pgto : <<HTTPS / REST>>
@enduml