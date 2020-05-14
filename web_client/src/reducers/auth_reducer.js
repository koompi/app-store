export const USER = "USER";
export const MAINTAINER = "MAINTAINER";
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

		case MAINTAINER:
			UpdateToken({ email: "maintaine@koompi.com", role: MAINTAINER });
			return { email: "maintainer@koompi.com", role: MAINTAINER };

		case ADMIN:
			UpdateToken({ email: "admin@koompi.com", role: ADMIN });
			return { email: "admin@koompi.com", role: ADMIN };
		default:
			UpdateToken({ email: "guest@koompi.com", role: GUEST });
			return { email: "guest@koompi.com", role: GUEST };
	}
};
