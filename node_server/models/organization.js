const mongoose = require("mongoose");
const Schema = mongoose.Schema;

const ORG = new Schema({
	name: String,
	description: String,
	owner_id: String,
	members: [String],
});

module.exports = mongoose.model("ORG", ORG);
