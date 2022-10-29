---
markmap:
  colorFreezeLevel: 3
---

# Rust

## Basic Data Types

- Booleans
    - True ou 1
    - False ou 0

- Char
    - Tem tamaho de 4 bytes

- Float
  - f32
  - f64
- Integers
   - Unsigned
     - Lado esquerdo se dizendo que é um Unsigned e lado direito o numero de bits Ex :
u8, u16, u32, u64, u128
     - Tipos de inteiros sem sinal ou seja somente positivos
   - Signed
     - Lado esquerdo se dizendo que é um Signed e lado direito o numero de bits Ex:
 i8, i16, i32, i64, i128
     - Tipos de Inteiros com sinal ou seja positivos e negativos
   - Special Types
     - São representados dependendo da arquitetura do sistema, que pode ser 32 ou 64 bits (usize / isize).


## Memory
- Stack
  - Management
    - Quando uma função é finalizada o espaço na stack é liberado.
    - Não precisamos nos preocupar em liberar a stack.
    - O Tamanho  da memoria  é limitado por arquitetura, sistema operacional e outros fatores, se nós atingirmos o limite da stack, teremos uma Exception stackOveflow.
  -  O tamanho de cada variável  na stack é conhecido em tempo de compilação
  - Aloca as variáveis criadas por cada função de forma empilhada. A cada chamada de uma função é alocado no top da pilha.
  - Variáveis acessadas somente pela função que a criou.
- Smart Pointer
  - Isso nos  permite desalocar as variáveis que estão  alocadas na Heap
  - Ex: SMART_POINTER(7) o endereço para o valor 7 na HEAP  será desalocado apos o termino da função.
  - Ex: Uso  na linguagem let e = Box::new(7);
- Heap
  - Management
    - Região da memoria que não é  gerenciado automaticamente, a alocação e desalocação é feita manualmente por nós.
    - Tem seu tamanho ilimitado sem restrições,  é somente limitado pelo tamanho da memoria física do sistema
  - Os endereços das variáveis são acessíveis por qualquer função de qualquer lugar no programa.
  - Alocação  na heap é custoso e devemos evitar quando possível, porque isso faz muitas alocações e fragmentação da heap sendo necessário encontrar novos espaços para novas locações.
  - A não desalocação de memoria irá gerar um memory leak, somente irá ser liberado no termino do programa.
## What is Rust
 - Faz uso eficiente de memória
 - Não tem runtime
 - Objetivo da linguagem
 - Fast
 - Memory-safety
     - Não ocorre Buffer Overflow
     - Não possui Null pointer deference
     - Não faz uso de memoria não alocada
     - Não possui falha na liberação do ponteiro (free)
     - Liberação ilegal do ponteiro
 - Thread-safety
 - Não tem garbage collector
 - Não possui Referencia Nula
 - Não possui Exceptions
 - Sem Data race
## Rustup Instalador Official
  - Instala toda a Toolchain necessaria pra compilar os programas em rust
  - Comandos de instalação
       
    -
        ```command
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        ```
  - Utilizado pra  fazer o upgrade do rust
## Cargo
   - Gerenciador de  pacotes do rust
   - Faz o download das dependências
   - Compila os pacotes
   - Faz os pacotes distribuíveis
   - Comandos
   
     -
       ```code
         Cargo new <nameProject> #Cria um novo projeto
         ```
     -
       ```code
         Cargo build #Faz o build 
        ```
     -
       ```code
         Cargo run #Executa o programa 
       ```
     -
       ```code
         Cargo install <nomeDaDependencia> #Instala alguma dependencia
       ```
## Common Programming Concepts
   - Data Types
   - Function
   - Comments
   - Control Flow
## Ownership
  - What Is Ownership?
  - References and Borrowing
  - The Slice Type

## Related Projects

- [coc-markmap](https://github.com/gera2ld/coc-markmap)
- [gatsby-remark-markmap](https://github.com/gera2ld/gatsby-remark-markmap)

## Features

- links
- **strong** ~~del~~ *italic* ==highlight==
- multiline
  text
- `inline code`
-
    ```js
    console.log('code block');
    ```
- Katex
  - $x = {-b \pm \sqrt{b^2-4ac} \over 2a}$
  - [More Katex Examples](#?d=gist:af76a4c245b302206b16aec503dbe07b:katex.md)
- Now we can wrap very very very very long text based on `maxWidth` option


