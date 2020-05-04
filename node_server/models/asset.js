const mongoose = require("mongoose");
const Schema = mongoose.Schema;

const ASSET = new Schema({
	name: String,
	type: String,
	date_created: String,
	modified_date: String,
	owner_id: String,
	privacy: String,
	description: String,
});

module.exports = mongoose.model("ASSET", ASSET);
