use yew::prelude::*;

#[function_component]
fn hello_world() -> Html {
    html!(

            //frist dv
            <div class="bg-black h-screen">

            //Title
            <div class="h-10 text-center font-mono bg-[#FE5000]">
              <p class="text-white ">{"AJUSTAR GUIA DE PROGRAMACION CANAL 19"}</p>
            </div>

            //Container from and video
            <div class="flex grid grid-cols-2  shadow-xl">
             
            //Container form
            <div name="divprimario" class="w-1/1 ">
            <div class="py-8  flex items-center justify-center h-full bg-black">
            
            //form
            <div name="divprimario2" class=" rounded-lg bg-[#474B4E] bg-opacity-50  place-content-center mx-auto">

            <p class="text-white mt-6 mx-8">{"Nombre del programa"}</p>
            <input class="rounded-lg border-solid border-2 border-sky-500 mt-3 h-10 w-80 mx-8" type="text" name="inp_name" />

            <p class="text-white mx-8">{"Descripcion del programa"}</p>
            <textarea class="rounded-lg border-solid border-2 border-sky-500 mt-3 h-40 w-80 mx-8" name="inp_descrip"></textarea>

            <p class="text-white mx-8">{"Clave del Video"}</p>
            <input class="rounded-md border-solid border-2 border-sky-500 h-10 mt-3 mx-8" type="text" name="inp_hora" />

            <p class="text-white mx-8">{"Hora de transmision"}</p>
            <input class="rounded-md border-solid border-2 border-sky-500 h-10 mt-3 mx-8" type="text" name="inp_date" />
            

            <div class=" mb-8  mt-5 mx-8 ">
                <button class="py-2 px-4 bg-blue-500 text-white font-semibold rounded-lg shadow-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-400 focus:ring-opacity-75" name="btn_add">{"Add"}</button>
            <div>
   //fin
         </div>
        </div>
     </div>
   </div>
</div>
            //Movie
            <div class=" h-full bg-black W-1/2">
                <iframe width="100%" height="100%" src="https://www.youtube.com/embed/sOuCHMGu5lI?autoplay=1" 
                   title="YouTube video player" frameborder="0" allow="autoplay" >
                </iframe>
            </div>

        </div>
           
            //Programing
     <div class="box_body_down my-8">
            <div class="bg-black">
              //head
               <div class="bg-white mx-20 h-20 flex flex-row  justify-center items-center space-x-40">
                 <div class="flex-grow">  <p class="text-black  text-center">{"PROGRAMA"}</p></div>
                 <div class="flex-grow">  <p class="text-black text-center ">{"DESCRIPCION"}</p></div>
                 <div class="flex-grow">  <p class="text-black  text-center">{"CLAVE"}</p></div>
                 <div class="flex-grow">  <p class="text-black text-center">{"HORA"}</p></div>
               </div>

             //Series
               <div class="bg-[#474B4E] border-solid border-2 border-[#FE5000] bg-opacity-70 mx-20 h-10 flex flex-row  my-8 justify-center items-center space-x-40">
                 <div class="flex-grow">  <p class="text-[#FE5000] text-center">{"Avatar"}</p></div>
                 <div class="flex-grow">  <p class="text-[#FE5000] text-center">{"C1|T1 El niño en el aciberg"}</p></div>
                 <div class="flex-grow">  <p class="text-[#FE5000] text-center">{"B100000"}</p></div>
                 <div class="flex-grow">  <p class="text-[#FE5000] text-center">{"AHORA"}</p></div>
               </div>

               <div class="bg-[#474B4E] bg-opacity-70 mx-20 h-10 flex flex-row  my-8 justify-center items-center space-x-40">
                 <div class="flex-grow">  <p class="text-white text-center">{"bob esponja"}</p></div>
                 <div class="flex-grow">  <p class="text-white text-center">{"C12|T21 Percebes"}</p></div>
                 <div class="flex-grow">  <p class="text-white text-center">{"B200000"}</p></div>
                 <div class="flex-grow">  <p class="text-white text-center">{"1:00 PM"}</p></div>
               </div>
               <div class="bg-[#474B4E] bg-opacity-70 mx-20 h-10 flex flex-row  my-8 justify-center items-center space-x-40">
                 <div class="flex-grow">  <p class="text-white text-center">{"bob esponja"}</p></div>
                 <div class="flex-grow">  <p class="text-white text-center">{"C22|T11 Kascarudolandia"}</p></div>
                 <div class="flex-grow">  <p class="text-white text-center">{"B200000"}</p></div>
                 <div class="flex-grow">  <p class="text-white text-center">{"1:30 PM PM"}</p></div>
               </div>
               <div class="bg-[#474B4E] bg-opacity-70 mx-20 h-10 flex flex-row  my-8 justify-center items-center space-x-40">
                 <div class="flex-grow">  <p class="text-white text-center">{"Hey Arnold"}</p></div>
                 <div class="flex-grow">  <p class="text-white text-center">{"C7|T2 El Comienzo"}</p></div>
                 <div class="flex-grow">  <p class="text-white text-center">{"B400000"}</p></div>
                 <div class="flex-grow">  <p class="text-white text-center">{"2:00 PM"}</p></div>
               </div>

            </div>

      </div>

</div>

        )
}

fn main() {
    yew::Renderer::<hello_world>::new().render();
}
