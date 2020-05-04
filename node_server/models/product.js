const mongoose = require("mongoose");
const Schema = mongoose.Schema;

const PRODUCT = new Schema({
	name: String,
	description: String,
	type: String,
	owner_id: String,
	date_created: String,
	status: String,
});

module.exports = mongoose.model("PRODUCT", PRODUCT);
