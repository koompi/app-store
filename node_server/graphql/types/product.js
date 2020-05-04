const graphql = require("graphql");
const { GraphQLString, GraphQLObjectType } = graphql;
const ProductType = new GraphQLObjectType({
	name: "ProductType",
	fields: {
		id: { type: GraphQLString },
		name: { type: GraphQLString },
		description: { type: GraphQLString },
		type: { type: GraphQLString },
		owner_id: { type: GraphQLString },
		date_created: { type: GraphQLString },
		status: { type: GraphQLString },
	},
});

module.exports = ProductType;
