use juniper::{FieldResult, EmptyMutation, Variables};

#[derive(juniper::GraphQLEnum, Clone, Copy)]
enum Episode{
    NewHope,
    Empire,
    Jedi
}

struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

struct Context(Episode);

impl juniper::Context for Context {
    
}

struct Query;
#[juniper::object(Context = Context)]
impl Query {
    fn favoriteEpisode(context: &Context) -> FieldResult<Episode> {
        Ok(context.0)
    }
}

type Schema = juniper::RootNode<'static, Query, EmptyMutation<Context>>;

fn main() {
  // Create a context object.
  let ctx = Context(Episode::NewHope);

  // Run the executor.
  let (res, _errors) = juniper::execute(
      "query { favoriteEpisode }",
      None,
      &Schema::new(Query, EmptyMutation::new()),
      &Variables::new(),
      &ctx,
  ).unwrap();

  // Ensure the value matches.
 }
