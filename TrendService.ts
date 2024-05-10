import axios, { AxiosError } from 'axios';

class ApiService {
  private baseUrl: string = process.env.REACT_APP_API_URL || '';

  private handleError(error: any): never {
    if (error.response) {
      console.error('Error data:', error.response.data);
      console.error('Error status:', error.response.status);
      console.error('Error headers:', error.response.headers);
      throw new Error(`Error: ${error.response.status} ${error.message}`);
    } else if (error.request) {
      console.error('Error request:', error.request);
      throw new Error('Error: The request was made but no response was received');
    } else {
      console.error('Error message:', error.message);
      throw new Error('Error: ' + error.message);
    }
  }

  async fetchTrendData(filters = {}): Promise<any> {
    try {
      const response = await axios.get(`${this.baseUrl}/trends`, { params: filters });
      return response.data;
    } catch (error) {
      this.handleError(error);
    }
  }

  async sendUserQuery(query: string): Promise<any> {
    try {
      const response = await axios.post(`${this.baseUrl}/user-query`, { query });
      return response.data;
    } catch (error) {
      this.handleError(error);
    }
  }

  async manageSubscription(domain: string, action: 'subscribe' | 'unsubscribe'): Promise<any> {
    try {
      const response = await axios.post(`${this.baseUrl}/subscriptions`, { domain, action });
      return response.data;
    } catch (error) {
      this.handleError(error);
    }
  }
}

export const apiService = new ApiService();