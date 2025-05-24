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
    event: 'stopped',
    data: {
      message : object;
    };
  }