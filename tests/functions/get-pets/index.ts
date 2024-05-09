// Follow this setup guide to integrate the Deno language server with your editor:
// https://deno.land/manual/getting_started/setup_your_environment
// This enables autocomplete, go to definition, etc.

console.log("Hello from Functions!")

enum PetType
{
  Dog,
  Cat
}

interface Pet
{
  name: string,
  age: number,
  animal: PetType
}

const pets: Pet[] = [
  {name: "Fido", age: 6, animal: PetType.Dog},
  {name: "Garfield", age: 12, animal: PetType.Cat},
  {name: "Whiskers", age: 3, animal: PetType.Cat},
  {name: "Bella", age: 5, animal: PetType.Dog},
  {name: "Shadow", age: 4, animal: PetType.Cat},
  {name: "Max", age: 7, animal: PetType.Dog},
  {name: "Luna", age: 2, animal: PetType.Cat},
  {name: "Charlie", age: 8, animal: PetType.Dog},
  {name: "Oliver", age: 1, animal: PetType.Cat},
  {name: "Daisy", age: 5, animal: PetType.Dog},
  {name: "Simba", age: 3, animal: PetType.Cat}
]

Deno.serve(async (req) => {
  const data = {
    pets: pets,
  }

  return new Response(
    JSON.stringify(data),
    { headers: { "Content-Type": "application/json" } },
  )
})

/* To invoke locally:

  1. Run `supabase start` (see: https://supabase.com/docs/reference/cli/supabase-start)
  2. Make an HTTP request:

*/
