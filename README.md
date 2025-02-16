# Nome do Projeto

> Descrição breve do projeto:  
> Uma aplicação desktop com backend em Rust utilizando Tauri, que expõe funcionalidades para o frontend via comandos. A aplicação é responsável por gerenciar tasks e tags associadas, seguindo boas práticas de design e uma estrutura modularizada para facilitar a manutenção e escalabilidade.

---

## Sumário

- [Visão Geral](#visão-geral)
- [Estrutura do Projeto](#estrutura-do-projeto)
  - [commands](#commands)
  - [database](#database)
  - [errors](#errors)
  - [models](#models)
  - [views](#views)
  - [utils](#utils)
  - [lib.rs e main.rs](#librs-e-mainrs)
  - [tests](#tests)
- [Fluxo do Backend](#fluxo-do-backend)
- [Feature: Criação de Tasks com Tags](#feature-criação-de-tasks-com-tags)
- [Gitflow e Fluxo de Branches](#gitflow-e-fluxo-de-branches)
- [Como Contribuir](#como-contribuir)
- [Licença](#licença)

---

## Visão Geral

Este projeto é uma aplicação desktop desenvolvida em Rust com Tauri, que integra um backend robusto e modular. O objetivo principal é fornecer uma interface simples e eficiente para a criação, gerenciamento e associação de tasks e tags. Utilizamos padrões de projeto que promovem a separação de preocupações, tornando o código mais legível, testável e escalável.

---

## Estrutura do Projeto

A seguir, detalhamos a responsabilidade de cada pasta e arquivo principal da estrutura do projeto:

### `commands`

- **Responsabilidade:**  
  - Atuar como ponto de entrada para o frontend via Tauri.  
  - Receber e validar os dados enviados pelo frontend.  
  - Invocar a camada de serviço ou lógica de negócio para processar as requisições.
  
- **Fluxo:**  
  `Frontend → Tauri Command → Service/Business Logic → Persistência/Outras Operações → Resposta ao Frontend`

### `database`

- **Responsabilidade:**  
  - Gerenciar a conexão com o banco de dados.  
  - Executar e controlar migrations.  
  - Implementar repositórios ou métodos de acesso a dados (queries, inserts, updates).

### `errors`

- **Responsabilidade:**  
  - Definir e centralizar tipos de erros customizados.  
  - Garantir uma formatação consistente e tratamento adequado de erros em toda a aplicação.

### `models`

- **Responsabilidade:**  
  - Definir as entidades do domínio (por exemplo, `Task` e `Tag`).  
  - Realizar o mapeamento entre as estruturas de dados internas e as tabelas do banco de dados.  
  - (Opcional) Incluir DTOs (Data Transfer Objects) para conversões entre camadas.

### `views`

- **Responsabilidade:**  
  - Formatar as respostas que serão enviadas ao frontend.  
  - Pode conter funções para serialização personalizada dos dados (por exemplo, para JSON).

### `utils`

- **Responsabilidade:**  
  - Armazenar funções utilitárias e helpers compartilhados entre módulos.  
  - Exemplos: funções de conversão, formatação, logging auxiliar, etc.

### `lib.rs` e `main.rs`

- **lib.rs:**  
  - Atua como ponto de entrada para a biblioteca interna, agrupando e exportando os módulos necessários.
  
- **main.rs:**  
  - É o entry point da aplicação.  
  - Inicializa o Tauri, configura os comandos e realiza a integração com outras partes, como a conexão com o banco de dados e leitura de configurações.

### `tests`

- **Responsabilidade:**  
  - Conter testes unitários e de integração para as funcionalidades implementadas.  
  - A estrutura dos testes espelha a organização dos módulos para facilitar a manutenção e a correlação entre testes e funcionalidades.

---

## Fluxo do Backend

O fluxo de processamento de uma requisição, como a criação de uma task, ocorre da seguinte forma:

1. **Entrada de Dados:**  
   - O frontend envia dados para um comando Tauri.
   - O módulo `commands` recebe os dados e realiza as validações iniciais.

2. **Chamada à Lógica de Negócio:**  
   - O comando invoca a camada de serviço (por exemplo, `task_service`), que contém as regras de negócio para criar uma task e gerenciar tags.
   - São realizadas validações adicionais, como verificação de duplicidades e integridade dos dados.

3. **Interação com o Banco de Dados:**  
   - A camada de serviço utiliza os repositórios definidos na pasta `database` para persistir as informações.
   - Caso seja necessário, operações envolvendo múltiplas tabelas (por exemplo, criação de task e associação de tags) são executadas dentro de uma transação para garantir atomicidade.

4. **Tratamento de Erros e Resposta:**  
   - Em caso de erros (falha de conexão, violação de integridade, etc.), os erros são tratados e formatados utilizando o módulo `errors`.
   - Uma resposta adequada é construída (possivelmente utilizando helpers da pasta `views`) e enviada de volta ao frontend.

5. **Testes e Validação:**  
   - A pasta `tests` contém testes para garantir que as validações, a lógica de negócio e as operações de banco de dados funcionem corretamente.

---

## Feature: Criação de Tasks com Tags

Para implementar a criação de tasks com associação de tags, a estratégia envolve:

1. **Definição dos Modelos:**
   - **Task:** Inclui atributos como `id`, `title`, `description`, `created_at`, etc.
   - **Tag:** Possui atributos como `id`, `name`, etc.
   - **Relacionamento Many-to-Many:** Se necessário, crie uma tabela de junção (ex.: `task_tags`) para associar múltiplas tags a uma task.

2. **Lógica de Negócio na Camada de Serviço:**
   - Um serviço (por exemplo, `task_service`) orquestra:
     - A criação da task.
     - A verificação se cada tag já existe (evitando duplicidades) ou se deve ser criada.
     - A associação das tags à task, garantindo integridade com o uso de transações.

3. **Exposição via Commands:**
   - No módulo `commands`, um comando (ex.: `create_task`) recebe os dados do frontend.
   - Após validações, o comando invoca o serviço para processar a criação da task e a associação das tags.
   - O comando retorna uma resposta estruturada (sucesso ou erro) ao frontend.

4. **Validações e Tratamento de Erros:**
   - Validações são realizadas tanto no nível dos commands quanto na camada de serviço.
   - Erros são tratados e formatados utilizando o módulo `errors`, garantindo feedback consistente para o usuário.

---

## Gitflow e Fluxo de Branches

Para manter um fluxo de desenvolvimento organizado, adotamos o seguinte Gitflow:

1. **Branches Principais:**
   - **`main`**: Contém o código em produção (estável e revisado).
   - **`develop`**: Branch de integração onde as novas features são consolidadas antes de serem publicadas.

2. **Branches de Feature:**
   - **Nome sugerido:** `feature/task-tag-creation`
   - **Fluxo:**  
     - Crie a branch a partir de `develop`:  
       ```bash
       git checkout develop
       git checkout -b feature/task-tag-creation
       ```
     - Desenvolva toda a funcionalidade de criação de tasks com tags nesta branch.
     - Realize commits incrementais e descritivos.
     - Quando a feature estiver completa e testada, abra um Pull Request (PR) para mesclar com `develop`.

3. **Branches de Release (quando necessário):**
   - **Nome sugerido:** `release/v1.0.0` (ou outra versão conforme o planejamento)
   - **Fluxo:**  
     - Crie a branch a partir de `develop`:  
       ```bash
       git checkout develop
       git checkout -b release/v1.0.0
       ```
     - Realize ajustes finais e correções de bugs.
     - Após aprovação, faça merge da branch `release/v1.0.0` em `main` e também em `develop` para manter a sincronização.

4. **Branches de Hotfix:**
   - **Nome sugerido:** `hotfix/fix-critical-bug`
   - **Fluxo:**  
     - Crie a branch a partir de `main`:  
       ```bash
       git checkout main
       git checkout -b hotfix/fix-critical-bug
       ```
     - Realize a correção necessária.
     - Após a correção, faça merge tanto em `main` quanto em `develop`.

---

## Como Contribuir

1. **Fork e Clone:**  
   - Faça um fork do repositório e clone localmente.
   
2. **Criação de Branches:**  
   - Siga o fluxo de Gitflow descrito acima para criar branches de feature, release ou hotfix conforme necessário.
   
3. **Commits e Pull Requests:**  
   - Faça commits bem detalhados e crie um Pull Request para revisão.
   - Siga as convenções de código e os testes existentes.

4. **Testes:**  
   - Certifique-se de que todos os testes estão passando antes de enviar o Pull Request.

---

## Licença

Distribuído sob a licença [Nome da Licença]. Veja o arquivo [LICENSE](LICENSE) para mais informações.

---

## Contato

- **Seu Nome** – [seu.email@exemplo.com](mailto:seu.email@exemplo.com)
- Projeto Link: [https://github.com/seuusuario/nomedoprojeto](https://github.com/seuusuario/nomedoprojeto)


