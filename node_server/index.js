const express = require("express");
const GraphQLHTTP = require("express-graphql");
const mongoose = require("mongoose");
const cors = require("cors");
const schema = require("./graphql/root");
const auth = require("./utils/auth");
require("dotenv").config();

const app = express();

const {
	DB_USER,
	DB_PASSWORD,
	DB_HOST,
	DB_PORT,
	DB_NAME,
	ENV,
	DEV_ENV,
	PRO_ENV,
} = process.env;

let PORT = ENV === "PRO" ? PRO_ENV : DEV_ENV;

app.use(cors("*"));
app.use(express.json());
app.use(
	"/graphql",
	// auth,
	GraphQLHTTP({
		graphiql: true,
		schema: schema,
	})
);

const MONGO_URI = `mongodb://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}`;
const MONGO_CONF = {
	useNewUrlParser: true,
	useUnifiedTopology: true,
	useCreateIndex: true,
	useFindAndModify: false,
};

try {
	mongoose.connect(MONGO_URI, MONGO_CONF, (error, client) => {
		if (error) {
			console.error("Connection to DATABASE was unsuccessful.");
			console.error("Error:", error.message);
		}
		if (client) {
			console.log("Connected to DATABASE successfully.");
			app.listen(PORT, () => console.log(`Server is running on port: ${PORT}`));
		}
	});
} catch (error) {
	if (error) {
		throw error;
	}
}
