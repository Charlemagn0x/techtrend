import axios from 'axios';

class ApiService {
  private baseUrl: string = process.env.REACT_APP_API_URL || '';

  async fetchTrendData(filters = {}): Promise<any> {
    try {
      const response = await axios.get(`${this.baseUrl}/trends`, { params: filters });
      return response.data;
    } catch (error) {
      console.error('Error fetching trend data', error);
      throw error;
    }
  }

  async sendUserQuery(query: string): Promise<any> {
    try {
      const response = await axios.post(`${this.baseUrl}/user-query`, { query });
      return response.data;
    } catch (error) {
      console.error('Error sending user query', error);
      throw error;
    }
  }

  async manageSubscription(domain: string, action: 'subscribe' | 'unsubscribe'): Promise<any> {
    try {
      const response = await axios.post(`${this.baseUrl}/subscriptions`, { domain, action });
      return response.data;
    } catch (error) {
      console.error('Error managing subscription', error);
      throw error;
    }
  }
}

export const apiService = new ApiService();