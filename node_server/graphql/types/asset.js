const graphql = require("graphql");
const { GraphQLString, GraphQLObjectType } = graphql;
const AssetType = new GraphQLObjectType({
	name: "AssetType",
	fields: {
		id: { type: GraphQLString },
		name: { type: GraphQLString },
		type: { type: GraphQLString },
		date_created: { type: GraphQLString },
		modified_date: { type: GraphQLString },
		owner_id: { type: GraphQLString },
		privacy: { type: GraphQLString },
		description: { type: GraphQLString },
	},
});

module.exports = AssetType;
