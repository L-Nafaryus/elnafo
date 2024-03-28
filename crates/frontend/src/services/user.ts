import api_client from "@/api-client";

class User {
    async register(login: string, email: string, password: string): Promise<JSON> {
        return await api_client.post("/user/register", JSON.stringify({ login: login, email: email, password: password }));
    }

    async login(email: string, password: string): Promise<JSON> {
        return await api_client.post("/user/login", JSON.stringify({ email: email, password: password }));
    }

    async logout(): Promise<JSON> {
        return await api_client.get("/user/logout");
    }

    async get(login: any): Promise<JSON> {
        return await api_client.get(`/user/${login}`);
    }

    async current(): Promise<JSON> {
        return await api_client.get("/user/current");
    }
}

export default new User();
