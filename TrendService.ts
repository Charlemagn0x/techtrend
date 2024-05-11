import axios, { AxiosError, AxiosResponse } from 'axios';

class ApiService {
  private baseUrl: string = process.env.REACT_APP_API_URL || '';

  private async makeRequest<T>(
    method: 'GET' | 'POST',
    url: string,
    data: object = {},
    params: object = {}
  ): Promise<T> {
    try {
      let response: AxiosResponse<T>;
      if (method === 'GET') {
        response = await axios.get<T>(`${this.baseUrl}/${url}`, { params });
      } else {
        response = await axios.post<T>(`${this.baseUrl}/${url}`, data);
      }
      return response.data;
    } catch (error) {
      this.handleError(error);
    }
  }

  private handleError(error: AxiosError): never {
    if (error.response) {
      console.error('Error data:', error.response.data);
      console.error('Error status:', error.response.status);
      console.error('Error headers:', error.response.headers);
      throw new Error(`Error: ${error.response.status} ${error.message}`);
    } else if (error.request) {
      console.error('Error request:', error.request);
      throw new Error('Error: The request was made but no response was received.');
    } else {
      console.error('Error message:', error.message);
      throw new Error('Error: ' + error.message);
    }
  }

  async fetchTrendData(filters: Record<string, any> = {}): Promise<any> {
    return this.makeRequest('GET', 'trends', {}, filters);
  }

  async sendUserQuery(query: string): Promise<any> {
    return this.makeRequest('POST', 'user-query', { query });
  }

  async manageSubscription(domain: string, action: 'subscribe' | 'unsubscribe'): Promise<any> {
    return this.makeRequest('POST', 'subscriptions', { domain, action });
  }
}

export const apiService = new ApiService();