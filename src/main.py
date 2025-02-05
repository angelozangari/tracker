import src.crud as crud
import src.speech as speech

def main():
    print("Hello from tracker!")
    
    while(True):
        #speech to text
        result = speech.speech_to_text()

        #parse command and execute function
        match result:
            case '1':
                crud.create_task()
            case '2':
                crud.read_task()
            case '3':
                crud.update_task()
            case '4':
                crud.delete_task()

        # text to speach
        speech.text_to_speech()


if __name__ == "__main__":
    main()
