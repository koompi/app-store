const mongoose = require("mongoose");
const Schema = mongoose.Schema;

const ROLE = new Schema({
	name: String,
	description: String,
});

module.exports = mongoose.model("ROLE", ROLE);
