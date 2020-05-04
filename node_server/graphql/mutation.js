const graphql = require("graphql");
const { GraphQLObjectType, GraphQLNonNull, GraphQLString } = graphql;
const Types = require("./types");
const MODELS = require("../models");
const {
	AssetType,
	OrgType,
	ProductType,
	ProfileType,
	RoleType,
	UserType,
} = Types;
const { ASSET, ORG, PRODUCT, PROFILE, ROLE, USER } = MODELS;
const NonNullString = { type: new GraphQLNonNull(GraphQLString) };
const String = { type: GraphQLString };
const Mutation = new GraphQLObjectType({
	name: "Mutation",
	fields: {
		create_user: {
			type: UserType,
			args: {
				name: NonNullString,
				email: NonNullString,
				password: NonNullString,
				profile_id: NonNullString,
				role: NonNullString,
			},
			resolve: async (root, args, context) => {
				let new_user = await new USER({ ...args }).save();
				return new_user;
			},
		},
		upate_user: {
			type: UserType,
			args: {
				id: NonNullString,
				email: String,
				password: String,
				profile_id: String,
				role: String,
			},
			resolve: async (root, args, context) => {
				let { id, ...data } = args;
				let update = await USER.findOneAndUpdate({ _id: id }, { ...data });
				return update;
			},
		},
		delete_user: {
			type: UserType,
			args: {
				id: NonNullString,
			},
			resolve: async (root, args, context) => {
				return await USER.findByIdAndDelete(args.id);
			},
		},
		// =========== Profile ============
		create_profile: {
			type: ProfileType,
			args: {
				first_name: NonNullString,
				last_name: NonNullString,
				date_of_birth: String,
				gender: String,
				address: String,
				avatar: String,
				cover: String,
			},
			resolve: async (root, args, context) => {
				let new_profile = await new PROFILE({ ...args }).save();
				return new_profile;
			},
		},
		update_profile: {
			type: ProfileType,
			args: {
				id: NonNullString,
				first_name: String,
				last_name: String,
				date_of_birth: String,
				gender: String,
				address: String,
				avatar: String,
				cover: String,
			},
			resolve: async (root, args, context) => {
				let { id, ...data } = args;
				let update = await PROFILE.findOneAndUpdate({ _id: id }, { ...data });
				return update;
			},
		},
		delete_profile: {
			type: ProfileType,
			args: {
				id: NonNullString,
			},
			resolve: async (root, args, context) => {
				return await PROFILE.findByIdAndDelete(args.id);
			},
		},
		// =========== Org =============
		create_org: {
			type: OrgType,
			args: {
				name: NonNullString,
				description: NonNullString,
				owner_id: NonNullString,
				members: NonNullString,
			},
			resolve: async (root, args, context) => {
				let new_org = await new ORG({ ...args }).save();
				return new_org;
			},
		},
		update_org: {
			type: OrgType,
			args: {
				id: NonNullString,
				name: String,
				description: String,
				owner_id: String,
				members: String,
			},
			resolve: async (root, args, context) => {
				let { id, ...data } = args;
				let update = await ORG.findOneAndUpdate({ _id: id }, { ...data });
				return update;
			},
		},
		delete_org: {
			type: OrgType,
			args: {
				id: NonNullString,
			},
			resolve: async (root, args, context) => {
				return await ORG.findByIdAndDelete(args.id);
			},
		},
		// ========== Product ==========
		create_product: {
			type: ProductType,
			args: {
				name: NonNullString,
				description: NonNullString,
				type: NonNullString,
				owner_id: NonNullString,
				date_created: NonNullString,
				status: NonNullString,
			},
			resolve: async (root, args, context) => {
				let new_product = await new PRODUCT({ ...args }).save();
				return new_product;
			},
		},
		update_product: {
			type: ProductType,
			args: {
				id: NonNullString,
				name: String,
				description: String,
				type: String,
				owner_id: String,
				date_created: String,
				status: String,
			},
			resolve: async (root, args, context) => {
				let { id, ...data } = args;
				let update = await PRODUCT.findOneAndUpdate({ _id: id }, { ...data });
				return update;
			},
		},
		delete_product: {
			type: ProductType,
			args: {
				id: NonNullString,
			},
			resolve: async (root, args, context) => {
				return await PRODUCT.findByIdAndDelete(args.id);
			},
		},
		// =========== Role ===========
		create_role: {
			type: RoleType,
			args: {
				name: NonNullString,
				description: NonNullString,
			},
			resolve: async (root, args, context) => {
				let new_role = await new ROLE({ ...args }).save();
				return new_role;
			},
		},
		update_role: {
			type: RoleType,
			args: {
				id: NonNullString,
				name: String,
				description: String,
			},
			resolve: async (root, args, context) => {
				let { id, ...data } = args;
				let update = await ROLE.findOneAndUpdate({ _id: id }, { ...data });
				return update;
			},
		},
		delete_role: {
			type: RoleType,
			args: {
				id: NonNullString,
			},
			resolve: async (root, args, context) => {
				return ROLE.findByIdAndDelete(args.id);
			},
		},
		// ========== Assets ============
		create_asset: {
			type: AssetType,
			args: {
				name: NonNullString,
				type: NonNullString,
				date_created: NonNullString,
				modified_date: NonNullString,
				owner_id: NonNullString,
				privacy: NonNullString,
				description: NonNullString,
			},
			resolve: async (root, args, context) => {
				let new_asset = await new ASSET({ ...args }).save;
				return new_asset;
			},
		},
		update_asset: {
			type: AssetType,
			args: {
				id: NonNullString,
				name: String,
				type: String,
				date_created: String,
				modified_date: String,
				owner_id: String,
				privacy: String,
				description: String,
			},
			resolve: async (root, args, context) => {
				let { id, ...data } = args;
				let update = await ASSET.findOneAndUpdate({ _id: id }, { ...data });
				return update;
			},
		},
		delete_asset: {
			type: AssetType,
			args: {
				id: NonNullString,
			},
			resolve: async (root, args, context) => {
				return ASSET.findByIdAndDelete(args.id);
			},
		},
	},
});

module.exports = Mutation;
