const graphql = require("graphql");
const { GraphQLString, GraphQLObjectType } = graphql;
const RoleType = new GraphQLObjectType({
	name: "RoleType",
	fields: {
		id: { type: GraphQLString },
		name: { type: GraphQLString },
		description: { type: GraphQLString },
	},
});

module.exports = RoleType;
