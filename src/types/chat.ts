export interface ChatMessage {
  text: string;
  isUser: boolean;
  isLoading?: boolean;
}

export type ChatEvent =
  | {
    event: 'started',
    data: {
      question: string;
    };
  }
  | {
    event: 'outputing',
    data: {
      message: object;
    };
  }
  | {
    event: 'finished',
    data: {
      status: string;
    };
  }