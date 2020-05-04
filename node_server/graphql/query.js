const graphql = require("graphql");
const {
	GraphQLObjectType,
	GraphQLString,
	GraphQLList,
	GraphQLNonNull,
} = graphql;
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

const Query = new GraphQLObjectType({
	name: "Query",
	fields: {
		// ========= Users =========
		user_by_id: {
			type: UserType,
			args: {
				id: NonNullString,
			},
			resolve: async (root, args, context) => {
				let user = await USER.findById(args.id);
				return user;
			},
		},
		all_users: {
			type: GraphQLList(UserType),
			resolve: async (root, args, context) => {
				let users = await USER.find();
				return users;
			},
		},
		// ========= Profiles =========
		profile_by_id: {
			type: ProfileType,
			args: {
				id: NonNullString,
			},
			resolve: async (root, args, context) => {
				let profile = await PROFILE.findById(args.id);
				return profile;
			},
		},
		all_profiles: {
			type: GraphQLList(ProfileType),
			resolve: async (root, args, context) => {
				let profiles = await PROFILE.find();
				return profiles;
			},
		},
		// ========= Orgs =========
		org_by_id: {
			type: OrgType,
			args: {
				id: NonNullString,
			},
			resolve: async (root, args, context) => {
				let org = await ORG.findById(args.id);
				return org;
			},
		},
		all_orgs: {
			type: GraphQLList(OrgType),
			resolve: async (root, args, context) => {
				let orgs = await ORG.find();
				return orgs;
			},
		},
		// ========= Product =========
		product_by_id: {
			type: ProductType,
			args: {
				id: NonNullString,
			},
			resolve: async (root, args, context) => {
				let product = await PRODUCT.findById(args.id);
				return product;
			},
		},
		all_products: {
			type: GraphQLList(ProductType),
			resolve: async (root, args, context) => {
				let products = await PRODUCT.find();
				return products;
			},
		},
		// ========== Role ==========
		role_by_id: {
			type: RoleType,
			args: {
				id: NonNullString,
			},
			resolve: async (root, args, context) => {
				let role = await ROLE.findBy(args.id);
				return role;
			},
		},
		all_roles: {
			type: GraphQLList(RoleType),
			resolve: async (root, args, context) => {
				let roles = await ROLE.find();
				return roles;
			},
		},
		// =========== Asset =========
		asset_by_id: {
			type: AssetType,
			args: {
				id: NonNullString,
			},
			resolve: async (root, args, context) => {
				let asset = await ASSET.findById(args.id);
				return asset;
			},
		},
		all_assets: {
			type: GraphQLList(AssetType),
			resolve: async (root, args, context) => {
				let assets = await ASSET.find();
				return assets;
			},
		},
	},
});

module.exports = Query;
