const mongoose = require("mongoose");
const Schema = mongoose.Schema;

const PROFILE = new Schema({
	first_name: String,
	last_name: String,
	date_of_birth: String,
	gender: String,
	address: String,
	avatar: String,
	cover: String,
});

module.exports = mongoose.model("PROFILE", PROFILE);
