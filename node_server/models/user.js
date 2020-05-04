const mongoose = require("mongoose");
const Schema = mongoose.Schema;

const USER = new Schema({
	name: String,
	email: String,
	password: String,
	profile_id: String,
	role: String,
});

module.exports = mongoose.model("USER", USER);
