const graphql = require("graphql");
const { GraphQLString, GraphQLObjectType } = graphql;
const UserType = new GraphQLObjectType({
	name: "UserType",
	fields: {
		id: { type: GraphQLString },
		name: { type: GraphQLString },
		email: { type: GraphQLString },
		password: { type: GraphQLString },
		profile_id: { type: GraphQLString },
		role: { type: GraphQLString },
	},
});

module.exports = UserType;
