import schedule
import time
from delivery_forecast.forecasting import perform_forecast
# from infrastructure.repositories import get_forecast_interval


class ForecastScheduler:
    """
    A scheduler class for automating the execution of delivery forecasting at specified intervals.

    This class uses the 'schedule' library to schedule and run the forecasting job.

    Attributes:
        None

    Methods:
        __init__(): Initializes the scheduler and sets up the initial job based on the forecast interval.
        _job(): The job to be executed at each scheduled interval, performs the forecasting.
        run(): Starts the scheduler and runs it indefinitely.

    Usage:
        scheduler = ForecastScheduler()
        scheduler.run()
    """

    def __init__(self):
        """
        Initialize the ForecastScheduler.

        Sets up the initial forecasting job based on the forecast interval retrieved from the database.
        """
        import sys
        # print(sys.path)  # Print Python path for debugging
        initial_interval = 1
        schedule.every(initial_interval).hours.do(self._job)
        self._job()

    def _job(self):
        """
        The job to be executed at each scheduled interval.

        Performs the forecasting by calling the 'perform_forecast' function and prints a message.

        Returns:
            None
        """
        # interval = get_forecast_interval()

        # print(f"Running forecast job at {time.strftime('%Y-%m-%d %H:%M:%S', time.localtime())}")

        interval = 30

        perform_forecast()

        print(f"Forecast executed. Next forecast in {interval} hours.")

    def run(self):
        """
        Start the scheduler and run it indefinitely.

        The scheduler runs pending jobs at their scheduled intervals.

        Returns:
            None
        """

        print("Running scheduler")
        while True:
            schedule.run_pending()
            time.sleep(1)
