const graphql = require("graphql");
const { GraphQLString, GraphQLObjectType } = graphql;
const OrgType = new GraphQLObjectType({
	name: "OrgType",
	fields: {
		id: { type: GraphQLString },
		name: { type: GraphQLString },
		description: { type: GraphQLString },
		owner_id: { type: GraphQLString },
		members: { type: GraphQLString },
	},
});

module.exports = OrgType;
