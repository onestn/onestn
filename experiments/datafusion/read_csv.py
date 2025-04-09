from datafusion import SessionContext

ctx = SessionContext()

pokemon_df = ctx.read_csv('dataset/pokemon.csv')

pokemon_df.show()
