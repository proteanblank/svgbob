extern crate svgbob;
extern crate svg;

use svgbob::Grid;
use svgbob::Settings;


fn main() {
    let file = "image.svg";
    let g = Grid::from_str(get_arg());
    //let svg = g.get_svg(&Settings::default());
    //let svg = g.get_svg(&Settings::no_optimization());
    let svg = g.get_svg(&Settings::separate_lines());
    let svg = g.get_svg(&Settings::compact());
    svg::save(file, &svg).unwrap();
    println!("Saved to {}",file);
}

fn get_arg() -> &'static str{

let arg = r#"


+------+   +-----+   +-----+   +-----+
|      |   |     |   |     |   |     |
| Foo  +-->| Bar +---+ Baz |<--+ Moo |
|      |   |     |   |     |   |     |
+------+   +-----+   +--+--+   +-----+
              ^         |
              |         V
.-------------+-----------------------.
| Hello here and there and everywhere |
'-------------------------------------'


                        ____________
   .--------------.     \           \
  / a == b         \     \           \     __________
 (    &&            )     ) process   )    \         \
  \ 'string' ne '' /     /           /     / process /
   '--------------'     /___________/     /_________/


    __________________
    \_________________\
     \                 \
      . another process .
     /_________________/
    /_________________/

  User code  ^               ^ OS code
              \             /
               \        .--'
                \      /
  User code  <--- Mode ----> OS code
                /      \
            .--'        \___
           /                \
          v                  v 
       User code            OS code

             .---.  .---. .---.  .---.    .---.  .---.
    OS API   '---'  '---' '---'  '---'    '---'  '---'
               |      |     |      |        |      |
               v      v     |      v        |      v
             .------------. | .-----------. |  .-----.
             | Filesystem | | | Scheduler | |  | MMU |
             '------------' | '-----------' |  '-----'
                    |       |      |        |
                    v       |      |        v
                 .----.     |      |    .---------.
                 | IO |<----'      |    | Network |
                 '----'            |    '---------'
                    |              |         |
                    v              v         v
             .---------------------------------------.
             |                  HAL                  |
             '---------------------------------------'
             

   ____[]
  | ___ |
  ||   ||  device
  ||___||  loads
  | ooo |----------------------------------------------------------.
  | ooo |    |                          |                          |
  | ooo |    |                          |                          |
  '-----'    |                          |                          |
             |                          |                          |
             v                          v                          v
   .-------------------.  .---------------------------.  .-------------------.
   | Loadable module C |  |     Loadable module A     |  | Loadable module B |
   '-------------------'  |---------------------------|  |   (instrumented)  |
             |            |         .-----.           |  '-------------------'
             '------------+-------->| A.o |           |             |
                 calls    |         '-----'           |             |
                          |    .------------------.   |             |
                          |   / A.instrumented.o /<---+-------------'
                          |  '------------------'     |    calls
                          '---------------------------'   

        .--------------.
         \              \
          '--------------'

                                        .--> Base::Class::Derived_A
                                       /
                                      .----> Base::Class::Derived_B    
      Something -------.             /         \
                        \           /           .---> Base::Class::Derived
      Something::else    \         /             \
            \             \       /               '--> Base::Class::Derived
             \             \     /
              \             \   .-----------> Base::Class::Derived_C 
               \             \ /
                '------ Base::Class
                       /  \ \ \
                      '    \ \ \  
                      |     \ \ \
                      .      \ \ '--- The::Latest
                     /|       \ \      \
 With::Some::fantasy  '        \ \      '---- The::Latest::Greatest
                     /|         \ \
         More::Stuff  '          \ '- I::Am::Running::Out::Of::Ideas
                     /|           \
         More::Stuff  '            \
                     /              '--- Last::One
       More::Stuff  V 

                    /  \
                   /    \
                  /      \
                 /        \
  Safety
    ^
    |                       *Rust
    |           *Java
    | *Python
    |                        *C++
    +-----------------------------> Control

    ^
    :
    :
    :
    :
<===+==============================>
    :
    :
    V
    
    ..............................

    ...
    ..

  this is a sentence
  separated  words  of nill

  TODO:
        
      ^ ^ ^
       \|/
        . 
       /|\
      v V v 

      ^ ^ ^
       \|/
      <-+->
       /|\
      v V v 

       \|/
       -.-
       /|\

        |   \/   
       -+-  /\      
        |   
        
        |      |    |      |
        +--  --+    +--  --+   +--  --+
                    |      |   |      |

                     |    |  |     |
             .- -.   .-  -.  '-   -'
             |   |

        .-   -.  .-.       
        '-   -'  | |  | |  
                      '-'

      \      |    /  |
       .     '   '   .
       |    /    |    \ 

        \    / 
         .  .
        /    \

       .    .
      /|    |\

      
      \|   |/
       '   '

       \
       /

       /
       \


       /      \
      '--    --'
     /          \

       /   \
    --'     '--
     /       \

                       \         /
       --.--  --.--   --.--   --.--
        /        \     


        |   |
        .   .
       /|   |\ 

        |
        .
       / \

       \|/
        .
       /|\

       
       \|/
      --.--
       /|\

       \|/
      --+--
       /|\
        
        |/  \|
        .    .
        |    |


       -.  -.
       /     \

        .-  .-
       /     \

      
       /   /     \    \
      '-  '-------'   -'
       

       .-.
      (   )
       '-'

       ..
      (  )
       ''


       .------.
      (        )
       '------'

        ________  
       /       /
      /       /
     /_______/


        ________  
        \       \
         \       \
          \_______\

       ________ 
      |________|


       ________ 
      |        |
      |________|

      .-.
      '-'

        ________  
        \_______\

       /\
      /  \
     /____\

       /\
      /  \
     /    \
    '------'

       ___
      /   \
      \___/

      ______
     /      \
    /        \
    \        /
     \______/
        

        +---------+
        |         |                        +--------------+
        |   NFS   |--+                     |              |
        |         |  |                 +-->|   CacheFS    |
        +---------+  |   +----------+  |   |  /dev/hda5   |
                     |   |          |  |   +--------------+
        +---------+  +-->|          |  |
        |         |      |          |--+
        |   AFS   |----->| FS-Cache |
        |         |      |          |--+
        +---------+  +-->|          |  |
                     |   |          |  |   +--------------+
        +---------+  |   +----------+  |   |              |
        |         |  |                 +-->|  CacheFiles  |
        |  ISOFS  |--+                     |  /var/cache  |
        |         |                        +--------------+
        +---------+
    

"#;

arg
}
