import { createYoga, createSchema } from "graphql-yoga"
import resolvers from "./resolvers"
import { file } from "bun"

const yoga = createYoga({
	schema: createSchema({
		typeDefs: await file("src/schema.graphqls").text(),
		resolvers,
	}),
})

const server = Bun.serve(yoga)

console.info(
	`Server is running on ${new URL(
		yoga.graphqlEndpoint,
		`http://${server.hostname}:${server.port}`
	)}`
)
