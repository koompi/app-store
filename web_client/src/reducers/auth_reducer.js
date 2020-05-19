export const USER = "USER";
export const DEVELOPER = "DEVELOPER";
export const ADMIN = "ADMIN";
export const GUEST = "GUEST";

const UpdateToken = async (data) => {
	window.localStorage.setItem("token", JSON.stringify(data));
};

export const auth_reducer = (state, action) => {
	switch (action.type) {
		case USER:
			UpdateToken({ email: "user@koompi.com", role: USER });
			return { email: "user@koompi.com", role: USER };

		case DEVELOPER:
			UpdateToken({ email: "maintaine@koompi.com", role: DEVELOPER });
			return { email: "DEVELOPER@koompi.com", role: DEVELOPER };

		case ADMIN:
			UpdateToken({ email: "admin@koompi.com", role: ADMIN });
			return { email: "admin@koompi.com", role: ADMIN };
		default:
			UpdateToken({ email: "guest@koompi.com", role: GUEST });
			return { email: "guest@koompi.com", role: GUEST };
	}
};
