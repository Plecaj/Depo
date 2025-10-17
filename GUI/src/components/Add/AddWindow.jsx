import styles from './Add.module.css';

function AddWindow({isVisible, setIsVisible}) {

    return(
        <>
            {isVisible &&
                <div  className={styles.backGround}>
                    <div className={styles.window}>
                        <div className={styles.header}>
                            <button className={styles.closeButton} onClick={() => setIsVisible(false)}>X</button>
                        </div>
                    </div>
                </div>
            }
        </>
    );
}

export default AddWindow;
