const graphql = require("graphql");
const { GraphQLString, GraphQLObjectType } = graphql;
const ProfileType = new GraphQLObjectType({
	name: "ProfileType",
	fields: {
		id: { type: GraphQLString },
		first_name: { type: GraphQLString },
		last_name: { type: GraphQLString },
		date_of_birth: { type: GraphQLString },
		gender: { type: GraphQLString },
		address: { type: GraphQLString },
		avatar: { type: GraphQLString },
		cover: { type: GraphQLString },
	},
});

module.exports = ProfileType;
