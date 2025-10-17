import styles from'./Build.module.css';


function Build() {

    const build = () => {

    }

    return(
        <>
            <div className={styles.button} onClick={build} > Build </div>
        </>
    );
}

export default Build;

